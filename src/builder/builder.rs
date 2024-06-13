//房子【产品（Product）】
#[derive(Debug)]
struct House {
    foundation: Option<String>,
    structure: Option<String>,
    roof: Option<String>,
    interior: Option<String>,
}

impl House {
    fn new() -> Self {
        Self {
            foundation: None,
            structure: None,
            roof: None,
            interior: None,
        }
    }
    fn set_foundation(&mut self, foundation: String) {
        self.foundation = Some(foundation);
    }

    fn set_structure(&mut self, structure: String) {
        self.structure = Some(structure);
    }

    fn set_roof(&mut self, roof: String) {
        self.roof = Some(roof);
    }

    fn set_interior(&mut self, interior: String) {
        self.interior = Some(interior);
    }
}

//房子抽象构建者【抽象建造者（Abstract Builder）】
trait HouseBuilder {
    fn build_foundation(&mut self);
    fn build_structure(&mut self);
    fn build_roof(&mut self);
    fn build_interior(&mut self);
    fn get_house(&self) -> &House;
}

//房子具体构建者【具体建造者（Concrete Builder）】
#[derive(Debug)]
struct ConcreteHouseBuilder {
    house: House,
}

impl ConcreteHouseBuilder {
    fn new(house: House) -> Self {
        Self { house }
    }
}
//实现抽象建造者
impl HouseBuilder for ConcreteHouseBuilder {
    fn build_foundation(&mut self) {
        self.house.set_foundation("Standard Foundation".to_string());
    }
    fn build_structure(&mut self) {
        self.house.set_structure("Standard Structure".to_string());
    }
    fn build_roof(&mut self) {
        self.house.set_roof("Standard Roof".to_string());
    }
    fn build_interior(&mut self) {
        self.house.set_interior("Standard Interior".to_string());
    }
    fn get_house(&self) -> &House {
        &self.house
    }
}

//豪宅构建者Builder【具体建造者（Concrete Builder）】
#[derive(Debug)]
struct LuxuryHouseBuilder {
    house: House,
}

impl LuxuryHouseBuilder {
    fn new(house: House) -> Self {
        Self { house }
    }
}
//实现抽象建造者
impl HouseBuilder for LuxuryHouseBuilder {
    fn build_foundation(&mut self) {
        self.house.set_foundation("Strong  Foundation".to_string());
    }
    fn build_structure(&mut self) {
        self.house.set_structure("Reinforced Structure".to_string());
    }
    fn build_roof(&mut self) {
        self.house.set_roof("Reinforced Roof".to_string());
    }
    fn build_interior(&mut self) {
        self.house.set_interior("Luxury Interior".to_string());
    }
    fn get_house(&self) -> &House {
        &self.house
    }
}
//指导者（Director）
struct Director {
    builder: Box<dyn HouseBuilder>,
}

impl Director {
    fn new<T>(builder: T) -> Self
    where
        T: HouseBuilder + 'static,
    {
        Director {
            builder: Box::new(builder),
        }
    }
    //建造房子
    fn construct_house(&mut self) -> &House {
        self.builder.build_foundation();
        self.builder.build_structure();
        self.builder.build_roof();
        self.builder.build_interior();
        self.builder.get_house()
    }
}

#[test]
fn main() {
    let house1 = House::new();
    //建造普通房子
    let house_builder1 = ConcreteHouseBuilder::new(house1);
    let mut director1 = Director::new(house_builder1);
    //建造好的房子
    let house = director1.construct_house();
    println!("{:#?}", house);

    let house2 = House::new();
    //建造豪宅
    let house_builder2 = LuxuryHouseBuilder::new(house2);
    let mut director2 = Director::new(house_builder2);
    let house = director2.construct_house();
    println!("{:#?}", house);
}
