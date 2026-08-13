struct BurgerBuilder{
    components:Vec<BurgerComponent>,
}

enum BurgerComponent{
    BottomBun,
    Patty,
    Tomato,
    Chesse,
    Lettuce,
    TopBun,
}

impl BurgerBuilder {
    fn new()->self{
        self{
            components:vec![BurgerComponent:BottomBun],
        }
    }

    pub fn add_component(
        mut self , component :BurgerComponent
    )->BurgerBuilder{
        self.components.push(component);
    }
    
    pub fn build(mut self)->BurgerBuilder{
        self.components.push(BurgerComponent::TopBun)
        self
    }
}

fn main(){
    let burger = BurgerBuilder::new().add_component(BurgerComponent::Patty).add_component(BurgerComponent::Chesse);
}