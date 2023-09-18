use std::collections::HashMap;

use crate::libs::position::Position;

use super::attributes::Attribute;
#[derive(Hash,PartialEq)]
pub enum Roles {
   Goalkeeper,SweeperKeeper,
   FullBack,DefensivFullBack,WingBack,InvertedWingBack,
   CentralDefender,BallPlayingDefender,NoNonsenseCentrBack,Sweeper,Libero,
   DefensiveMidfielder,DeepLyingPlaymaker,Anchor,BallWinningMidfielder, RoamingPlaymaker,
   CentralMidfielder,BoxtoBoxMidfielder, AdvancePlaymaker,
   WideMidfielder,Winger,DefensiveWinger, InvertedWinger,
   InsideForward,AttackingMidfielder,ShadowStriker,Trequartiesta,
   PressingForward,
   Poacher,TargetForward,DeepLyingForward,AdvanceForward,CompleteForward
}
impl Eq for Roles {}

pub struct PositionRoles{}
impl PositionRoles {
    pub fn make()->HashMap<Position,Vec<Roles>> {
        let mut hm:HashMap<Position,Vec<Roles>> = HashMap::new();
        hm.insert(Position::GK, vec![Roles::Goalkeeper,Roles::SweeperKeeper]).unwrap();
        hm.insert(Position::DR, vec![Roles::FullBack,Roles::DefensivFullBack,Roles::WingBack,Roles::InvertedWingBack]);
        hm.insert(Position::DL, vec![Roles::FullBack,Roles::DefensivFullBack,Roles::WingBack,Roles::InvertedWingBack]);
        hm.insert(Position::DC, vec![Roles::CentralDefender,Roles::BallPlayingDefender,Roles::NoNonsenseCentrBack,Roles::Sweeper,Roles::Libero]);
        hm.insert(Position::DMC, vec![Roles::DefensiveMidfielder,Roles::DeepLyingPlaymaker,Roles::Anchor,Roles::BallWinningMidfielder,Roles::RoamingPlaymaker]);
        hm.insert(Position::WBR, vec![Roles::WingBack,Roles::InvertedWingBack]);
        hm.insert(Position::WBL, vec![Roles::WingBack,Roles::InvertedWingBack]);
        hm.insert(Position::MR, vec![Roles::WideMidfielder,Roles::Winger,Roles::DefensiveWinger,Roles::InvertedWinger]);
        hm.insert(Position::ML, vec![Roles::WideMidfielder,Roles::Winger,Roles::DefensiveWinger,Roles::InvertedWinger]);
        hm.insert(Position::MC, vec![Roles::CentralMidfielder,Roles::DeepLyingPlaymaker,Roles::Anchor,Roles::BallWinningMidfielder,Roles::BoxtoBoxMidfielder,Roles::AdvancePlaymaker,Roles::RoamingPlaymaker]);
        hm.insert(Position::AMC, vec![Roles::AttackingMidfielder,Roles::AdvancePlaymaker,Roles::Trequartiesta,Roles::ShadowStriker]);
        hm.insert(Position::AMR, vec![Roles::Winger,Roles::InsideForward,Roles::AdvanceForward,Roles::InvertedWinger]);
        hm.insert(Position::AML, vec![Roles::Winger,Roles::InsideForward,Roles::AdvanceForward,Roles::InvertedWinger]);
        hm.insert(Position::FL, vec![Roles::InsideForward,Roles::AdvanceForward,Roles::PressingForward]);
        hm.insert(Position::FR, vec![Roles::InsideForward,Roles::AdvanceForward,Roles::PressingForward]);
        hm.insert(Position::ST, vec![Roles::Poacher,Roles::TargetForward,Roles::DeepLyingForward,Roles::AdvanceForward,Roles::CompleteForward,Roles::PressingForward,Roles::Trequartiesta]);

        hm
    }
}
pub struct AttributesRole{}
impl AttributesRole {
    pub fn make()->HashMap<Roles,Vec<Attribute>>{
        let mut hm:HashMap<Roles,Vec<Attribute>> = HashMap::new();
        hm.insert(Roles::CentralDefender,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::CentralMidfielder, vec![Attribute::Aero,Attribute::Passing,Attribute::Movement,Attribute::Decision,Attribute::Position,Attribute::Tackle]);
        hm.insert(Roles::AdvanceForward, vec![Attribute::Movement,Attribute::Decision,Attribute::Shooting,Attribute::Pace]);
        hm

    }
} 
