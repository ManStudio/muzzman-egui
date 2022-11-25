use std::any::Any;

pub struct Storage {
    components: Vec<Box<dyn Any>>,
}

#[allow(dead_code)]
impl Storage {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, component: T) -> Option<Box<T>> {
        let mut res = None;
        {
            let mut index = 0;
            let mut finded = false;
            for (i, component) in self.components.iter().enumerate() {
                if component.downcast_ref::<T>().is_some() {
                    finded = true;
                    index = i;
                    break;
                }
            }
            if finded {
                res = Some(self.components.remove(index).downcast::<T>().unwrap());
            }
        }
        let component = Box::new(component);
        self.components.push(component);
        res
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let mut res = None;
        for component in self.components.iter() {
            if let Some(component) = component.downcast_ref::<T>() {
                res = Some(component);
                break;
            }
        }

        res
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let mut res = None;
        for component in self.components.iter_mut() {
            if let Some(component) = component.downcast_mut::<T>() {
                res = Some(component);
                break;
            }
        }

        res
    }

    pub fn remove<T: 'static>(&mut self) -> Option<Box<T>> {
        let mut res = None;
        {
            let mut index = 0;
            let mut finded = false;
            for (i, component) in self.components.iter().enumerate() {
                if component.downcast_ref::<T>().is_some() {
                    finded = true;
                    index = i;
                    break;
                }
            }
            if finded {
                res = Some(self.components.remove(index).downcast::<T>().unwrap());
            }
        }

        res
    }
}
