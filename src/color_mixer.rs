use std::{
    mem::MaybeUninit,
    ops::{Add, Mul},
};

use bevy::prelude::*;
use mixbox_sys::{
    mixbox_latent_to_srgb32f, mixbox_srgb32f_to_latent, MIXBOX_NUMLATENTS,
};

pub fn mix_colors(colors: &[Color]) -> Color {
    let weight = 1.0 / colors.len() as f32;
    colors
        .iter()
        .map(|color| Latent::from(color) * weight)
        .reduce(|accum, item| accum + item)
        .map_or(Color::NONE, |latent| Color::from(&latent))
}

#[derive(Debug)]
struct Latent([f32; Latent::LATENT_LENGTH]);

impl Latent {
    const LATENT_LENGTH: usize = MIXBOX_NUMLATENTS as _;

    pub fn new() -> Self {
        Self([0.0; Latent::LATENT_LENGTH])
    }
}

impl Mul<f32> for Latent {
    type Output = Latent;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Latent::new();
        for (i, _) in self.0.iter().enumerate() {
            result.0[i] = self.0[i] * rhs;
        }
        result
    }
}

impl Add<Latent> for Latent {
    type Output = Latent;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Latent::new();
        for (i, el) in self.0.iter().enumerate() {
            result.0[i] = el + rhs.0[i];
        }
        result
    }
}

impl From<&Latent> for Color {
    fn from(latent: &Latent) -> Self {
        let mut srgb = MaybeUninit::<[f32; 3]>::uninit();
        let srgb_ptr = srgb.as_mut_ptr().cast::<f32>();

        let srgb = unsafe {
            mixbox_latent_to_srgb32f(
                &latent.0 as *const _ as _,
                srgb_ptr.offset(0),
                srgb_ptr.offset(1),
                srgb_ptr.offset(2),
            );
            srgb.assume_init()
        };

        Color::rgb(srgb[0], srgb[1], srgb[2])
    }
}

impl From<&Color> for Latent {
    fn from(color: &Color) -> Self {
        let mut result = Latent::new();
        unsafe {
            mixbox_srgb32f_to_latent(
                color.r(),
                color.g(),
                color.b(),
                result.0.as_mut_ptr(),
            );
        }
        result
    }
}
