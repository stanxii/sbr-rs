#![feature(test)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate derive_builder;

extern crate csv;
extern crate ndarray;
extern crate rand;
extern crate rayon;
extern crate serde;
extern crate siphasher;
extern crate test;
extern crate wyrm;

pub mod data;
pub mod evaluation;
pub mod models;

pub type UserId = usize;
pub type ItemId = usize;
pub type Timestamp = usize;

pub trait OnlineRankingModel {
    type UserRepresentation: std::fmt::Debug;
    fn user_representation(
        &self,
        item_ids: &[ItemId],
    ) -> Result<Self::UserRepresentation, &'static str>;
    fn predict(
        &self,
        user: &Self::UserRepresentation,
        item_ids: &[ItemId],
    ) -> Result<Vec<f32>, &'static str>;
}
