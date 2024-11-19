use core::ops::{AddAssign, Div};

use crate::Renderer;


// TODO: This should be generic for the duration
pub struct Transition<T>{
    pub renderer: Renderer,
    duration: T,
    current: T,
}

impl<T: AddAssign + PartialOrd + Div<Output = T> + Default + Copy> Transition<T> {
    pub fn new(renderer: Renderer, duration: T) -> Self {
        Self {
            renderer,
            duration,
            current: T::default(),
        }
    }

    pub fn step(&mut self, dt: T) {
        self.current += dt;
    }

    pub fn is_done(&self) -> bool {
        self.current >= self.duration
    }

    pub fn progress(&self) -> T {
        if self.current >= self.duration || self.duration == T::default() {
            T::default()
        } else {
            self.current / self.duration
        }
    }
}