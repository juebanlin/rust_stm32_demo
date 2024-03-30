use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

///x y simpleAoi range
#[repr(C)]
pub struct Entity(f32, f32, f32);

impl Entity {
    fn getAoiX(&self) -> f32 {
        self.1
    }
    fn getAoiY(&self) -> f32 {
        self.0
    }
    fn getAoiRange(&self) -> f32 {
        self.2
    }
    pub fn new(x: f32, y: f32, range: f32) -> Entity {
        Entity(x, y, range)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"x:{},y:{},r:{}",self.0,self.1,self.2)
    }
}

pub struct AoiGroup {
    index: i32,
    drop: bool,
    entityids: HashSet<i32>,
}

impl AoiGroup {
    fn new(index: i32) -> AoiGroup {
        AoiGroup {
            index,
            drop: false,
            entityids: Default::default(),
        }
    }

    fn add(&mut self, index: i32) {
        self.entityids.insert(index);
    }
}
pub struct Aoi {
    gridWidth: f32,
    gridHeight: f32,
    wNum: i32,
    hNum: i32,
    num: i32,
    areas: Vec<Option<HashSet<i32>>>,
    entitys: Vec<Entity>,
    groups: Vec<AoiGroup>,
    id_group: HashMap<i32, i32>,
}

impl Aoi {
    pub fn new(
        entityNum: i32,
        worldWidth: f32,
        worldHeight: f32,
        gridWidth: f32,
        gridHeight: f32,
    ) -> Aoi {
        let gridwnum = (worldWidth / gridWidth).ceil() as i32;
        let gridhNum = (worldHeight / gridHeight).ceil() as i32;
        let gridnum = gridwnum * gridhNum;
        let mut areas_ = Vec::<Option<HashSet<i32>>>::with_capacity(gridhNum as usize);
        for _ in 0..gridnum {
            areas_.push(None);
        }
        Aoi {
            gridWidth,
            gridHeight,
            wNum: gridwnum,
            hNum: gridhNum,
            num: gridnum,
            areas: areas_,
            groups: Vec::with_capacity((entityNum / 2 + 1) as usize),
            entitys: Vec::with_capacity(entityNum as usize),
            id_group: HashMap::default(),
        }
    }

    pub fn buildResult(&self) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut nogroup = Vec::<i32>::new();
        let vec = &self.entitys;
        for (i, e) in vec.iter().enumerate() {
            let id = i as i32;
            let map = &self.id_group;
            let gid = map.get(&id).unwrap_or(&-1);
            if *gid < 0 {
                nogroup.push(id);
                continue;
            }
        }
        result.push(nogroup);
        for (i, g) in self.groups.iter().enumerate() {
            if g.drop {
                continue;
            }
            let mut group = Vec::<i32>::new();
            for x in g.entityids.iter() {
                group.push(*x);
            }
            result.push(group);
        }
        return result;
    }

    ///填充区域ID
    pub fn fillAreaId(&mut self, idx: &i32, id: &i32) {
        let area = self.areas.get_mut((*idx) as usize).unwrap();
        match area {
            Some(a) => {
                a.insert(*id);
            }
            None => {
                let mut newArea = HashSet::default();
                newArea.insert(*id);
                area.replace(newArea);
            }
        }
    }

    pub fn enter(&mut self, entity: Entity, id: i32) {
        self.entitys.push(entity);
        let e = self.entitys.get(id as usize).unwrap();
        let idxs = self.forInGrid(e, id);
        for idx in &idxs {
            self.fillAreaId(idx, &id);
        }
        self.addGroup(id);
    }

    pub fn addGroup(&mut self, id: i32) {
        let mut idTmps = HashSet::<i32>::default();
        let current = self.entitys.get(id as usize).unwrap();
        let idxs = self.forInGrid(current, id);
        for idx in &idxs {
            let mut area = self.areas.get(*idx as usize).unwrap();
            if let Some(a) = area {
                for id2 in a {
                    idTmps.insert(*id2);
                }
            }
        }
        if idTmps.len() < 0 {
            // 没有area，跳出
            return ();
        }
        for tid in idTmps {
            // 相同实体
            if tid == id {
                continue;
            }
            if Aoi::isSameGroup(&mut self.id_group, id, tid) {
                // 分组相同，不需要检测了
                continue;
            }
            let target = self.entitys.get(tid as usize);
            if target.is_none() {
                continue;
            }

            if Aoi::isCollision(current, target.unwrap()) {
                Aoi::mergeGroup(&mut self.groups, &mut self.id_group, &id, &tid);
            }
        }
    }

    pub fn forInGrid(&self, entity: &Entity, id: i32) -> Vec<i32> {
        let aoi = self;
        let x = entity.getAoiX();
        let y = entity.getAoiY();
        let xRange = entity.getAoiRange();
        let yRange = entity.getAoiRange();
        Aoi::forInGrid2(
            x,
            y,
            xRange,
            yRange,
            aoi.gridWidth,
            aoi.gridHeight,
            aoi.wNum,
            aoi.hNum,
        )
    }

    pub fn forInGrid2(
        x: f32,
        y: f32,
        xRange: f32,
        yRange: f32,
        gridWidth: f32,
        gridHeight: f32,
        wNum: i32,
        hNum: i32,
    ) -> Vec<i32> {
        let gridX = ((x / gridWidth).floor() as i32).max(0); //x轴格子坐标
        let gridY = ((y / gridHeight).floor() as i32).max(0); //y轴格子坐标
        //以格子的4个边界为起点的range(需要去掉range在格子中所占用的长度)
        let fixRange_Left = xRange - (x % gridWidth); //朝左的长度
        let fixRange_Right = xRange - (gridWidth - x % gridWidth); //朝右的长度
        let fixRange_Up = yRange - (gridHeight - y % gridHeight); //朝上的长度
        let fixRange_Down = yRange - (y % gridHeight); //朝下的长度
        //4个方向的边界延伸所占用的格子数
        let RangeXLNum = (fixRange_Left / gridWidth).ceil() as i32; //x左边
        let RangeXRNum = (fixRange_Right / gridWidth).ceil() as i32; //x右边
        let RangeYUNum = (fixRange_Up / gridHeight).ceil() as i32; //y上边
        let RangeYDNum = (fixRange_Down / gridHeight).ceil() as i32; //y下边
        //占用的所有格子的范围
        let ox = gridX - RangeXLNum;
        let oy = gridY - RangeYDNum;
        let ex = gridX + RangeXRNum;
        let ey = gridY + RangeYUNum;
        let mut vec = Vec::new();
        for gx in ox..=ex {
            for gy in oy..=ey {
                if gx < 0 || gx >= wNum || gy < 0 || gy >= hNum {
                    //越界
                    continue;
                }
                //格子数坐标转换为id索引
                let idx = gx + gy * wNum;
                vec.push(idx);
            }
        }
        return vec;
    }
    ///碰撞检测
    pub fn isCollision(a: &Entity, b: &Entity) -> bool {
        let dy = a.getAoiY() - b.getAoiY();
        let dx = a.getAoiX() - b.getAoiX();
        let dr = a.getAoiRange() + b.getAoiRange();
        (dx).powi(2).abs() + (dy).powi(2).abs() <= (dr).powi(2)
    }

    ///是否在同一组
    pub fn isSameGroup(id_group: &mut HashMap<i32, i32>, id1: i32, id2: i32) -> bool {
        let g1 = id_group.get(&id1);
        let g2 = id_group.get(&id2);
        g1.unwrap_or(&-1) == g2.unwrap_or(&-2)
    }

    ///放到一组
    pub fn mergeGroup(
        groups: &mut Vec<AoiGroup>,
        id_group: &mut HashMap<i32, i32>,
        id1: &i32,
        id2: &i32,
    ) {
        let selfG = *id_group.get(id1).unwrap_or(&-1);
        let otherG = *id_group.get(id2).unwrap_or(&-2);
        if selfG < 0 {
            if otherG < 0 {
                //双方都没有组,创建组
                let mut group = AoiGroup::new(groups.len() as i32);
                group.add(*id1);
                group.add(*id2);
                id_group.insert(*id1, group.index);
                id_group.insert(*id2, group.index);
                groups.push(group);
            } else {
                // g2加入对方组
                match groups.get_mut(otherG as usize) {
                    None => {}
                    Some(group) => {
                        group.add(*id1);
                        id_group.insert(*id1, group.index);
                    }
                }
            }
            return;
        }
        if otherG < 0 {
            // 对方没有组,对应加入我组
            // g2加入对方组
            match groups.get_mut(selfG as usize) {
                None => {}
                Some(group) => {
                    group.add(*id2);
                    id_group.insert(*id2, group.index);
                }
            }
        } else {
            //双方有组
            if selfG == otherG {
                // 同组，不做处理
                return;
            } else {
                let mut ids = vec![];
                {
                    let otherGroup = &groups.get(otherG as usize).unwrap().entityids;
                    for x in otherGroup {
                        ids.push(*x);
                    }
                }
                let selfGroup = groups.get_mut(selfG as usize).unwrap();
                for id in ids {
                    selfGroup.add(id);
                    id_group.insert(id, selfGroup.index);
                }
                groups.get_mut(selfG as usize).unwrap().drop = true;
            }
        }
    }
}