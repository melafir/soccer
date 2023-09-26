use std::{collections::HashMap, fmt::Display};

use crate::libs::position::Position;

use super::attributes::Attribute;
#[derive(Hash,PartialEq,Debug,Clone, Copy)]
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
impl<'a> Display for Roles{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut t = "".to_string();
        let mut  ten_string_check = move |i:&'a str|->&'a str{
            if i.len()>=15{
                &i[0..15]
            }else{
                t = i.to_string();
                for _ in 0..(15-i.len()){
                    t+=" ";
                }
                format!("{}",t).as_str()
            }
            
        };
        match self {
          Self::Goalkeeper => write!(f,"{}",ten_string_check("Goalkeeper")),
          Self::SweeperKeeper => write!(f,"{}",ten_string_check("Sweeper Keeper")),
          Self::FullBack => write!(f,"{}",ten_string_check("Full Back")),
          Self::DefensivFullBack => write!(f,"{}",ten_string_check("Defensive Full Back")),
          Self::WingBack => write!(f,"{}",ten_string_check("Wing Back")),
          Self::InvertedWingBack => write!(f,"{}",ten_string_check("Inverted Wing Back")),
          Self::CentralDefender => write!(f,"{}",ten_string_check("Central Defender")),
          Self::BallPlayingDefender => write!(f,"{}",ten_string_check("Ball Playing Defender")),
          Self::NoNonsenseCentrBack => write!(f,"{}",ten_string_check("No-Nonsense CentreBack")),
          Self::Sweeper => write!(f,"{}",ten_string_check("Sweeper")),
          Self::Libero => write!(f,"{}",ten_string_check("Libero")),
          Self::DefensiveMidfielder => write!(f,"{}",ten_string_check("Defensive Midfielder")),
          Self::DeepLyingPlaymaker => write!(f,"{}",ten_string_check("Deep Lying Playmaker")),
          Self::Anchor => write!(f,"{}",ten_string_check("Anchor")),
          Self::BallWinningMidfielder => write!(f,"{}",ten_string_check("Ball Winning Midfielder")),
          Self::RoamingPlaymaker => write!(f,"{}",ten_string_check("Roaming Playmaker")),
          Self::CentralMidfielder => write!(f,"{}",ten_string_check("Central Midfielder")),
          Self::BoxtoBoxMidfielder => write!(f,"{}",ten_string_check("Box to Box Midfielder")),
          Self::AdvancePlaymaker => write!(f,"{}",ten_string_check("Advance Playmaker")),
          Self::WideMidfielder => write!(f,"{}",ten_string_check("Wide Midfielder")),
          Self::Winger => write!(f,"{}",ten_string_check("Winger")),
          Self::DefensiveWinger => write!(f,"{}",ten_string_check("Defensive Winger")),
          Self::InvertedWinger => write!(f,"{}",ten_string_check("Inverted Winger")),
          Self::InsideForward => write!(f,"{}",ten_string_check("InsideForward")),
          Self::AttackingMidfielder => write!(f,"{}",ten_string_check("Attacking Midfielder")),
          Self::ShadowStriker => write!(f,"{}",ten_string_check("Shadow Striker")),
          Self::Trequartiesta => write!(f,"{}",ten_string_check("Trequartiesta")),
          Self::PressingForward => write!(f,"{}",ten_string_check("Pressing Forward")),
          Self::Poacher => write!(f,"{}",ten_string_check("Poacher")),
          Self::TargetForward => write!(f,"{}",ten_string_check("Target Forward")),
          Self::DeepLyingForward => write!(f,"{}",ten_string_check("Deep Lying Forward")),
          Self::AdvanceForward => write!(f,"{}",ten_string_check("Advance Forward")),
          Self::CompleteForward => write!(f,"{}",ten_string_check("Complete Forward")),
        }  
    }
}
impl Eq for Roles {}

pub struct PositionRoles{}
impl PositionRoles {
    pub fn make()->HashMap<Position,Vec<Roles>> {
        let mut hm:HashMap<Position,Vec<Roles>> = HashMap::new();
        hm.insert(Position::GK, vec![Roles::Goalkeeper,Roles::SweeperKeeper]);
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
#[derive(Debug)]
pub struct AttributesRole{}
impl AttributesRole {
    pub fn make()->HashMap<Roles,Vec<Attribute>>{
        let mut hm:HashMap<Roles,Vec<Attribute>> = HashMap::new();
        hm.insert(Roles::CentralDefender,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::BallPlayingDefender,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Passing,Attribute::Dribbling]);
        hm.insert(Roles::NoNonsenseCentrBack,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::Sweeper,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision]);
        hm.insert(Roles::Libero,vec![Attribute::Movement,Attribute::Decision,Attribute::Creative,Attribute::Shooting,Attribute::Pace]);
        hm.insert(Roles::FullBack,vec![Attribute::Tackle,Attribute::Position,Attribute::Pace,Attribute::Decision,Attribute::Passing]);
        hm.insert(Roles::DefensivFullBack,vec![Attribute::Tackle,Attribute::Position,Attribute::Pace,Attribute::Decision]);
        hm.insert(Roles::WingBack,vec![Attribute::Stamina,Attribute::Position,Attribute::Pace,Attribute::Crossing,Attribute::Dribbling]);
        hm.insert(Roles::InvertedWingBack,vec![Attribute::Tackle,Attribute::Position,Attribute::Pace,Attribute::Decision,Attribute::Passing]);
        hm.insert(Roles::DefensiveMidfielder,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength]);
        hm.insert(Roles::DeepLyingPlaymaker,vec![Attribute::Passing,Attribute::Creative,Attribute::Decision,Attribute::Movement]);
        hm.insert(Roles::Anchor,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength]);
        hm.insert(Roles::BallWinningMidfielder,vec![Attribute::Tackle,Attribute::Position,Attribute::Decision,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::RoamingPlaymaker,vec![Attribute::Passing,Attribute::Creative,Attribute::Decision,Attribute::Movement,Attribute::Shooting,Attribute::Teamwork]);
        hm.insert(Roles::CentralMidfielder, vec![Attribute::Aero,Attribute::Passing,Attribute::Movement,Attribute::Decision,Attribute::Position,Attribute::Tackle]);
        hm.insert(Roles::BoxtoBoxMidfielder,vec![Attribute::Movement,Attribute::Position,Attribute::Decision,Attribute::Stamina,Attribute::Tackle,Attribute::Passing]);
        hm.insert(Roles::AdvancePlaymaker,vec![Attribute::Passing,Attribute::Creative,Attribute::Decision,Attribute::Movement]);
        hm.insert(Roles::WideMidfielder,vec![Attribute::Passing,Attribute::Position,Attribute::Decision,Attribute::Movement,Attribute::Tackle]);
        hm.insert(Roles::Winger,vec![Attribute::Crossing,Attribute::Dribbling,Attribute::Decision,Attribute::Movement,Attribute::Pace]);
        hm.insert(Roles::DefensiveWinger,vec![Attribute::Crossing,Attribute::Tackle,Attribute::Decision,Attribute::Position,Attribute::Passing]);
        hm.insert(Roles::InvertedWinger,vec![Attribute::Passing,Attribute::Dribbling,Attribute::Decision,Attribute::Movement,Attribute::Pace]);
        hm.insert(Roles::AttackingMidfielder,vec![Attribute::Passing,Attribute::Decision,Attribute::Movement,Attribute::Shooting]);
        hm.insert(Roles::Trequartiesta,vec![Attribute::Movement,Attribute::Decision,Attribute::Creative,Attribute::Shooting,Attribute::Pace,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::ShadowStriker,vec![Attribute::Movement,Attribute::Decision,Attribute::Creative,Attribute::Shooting,Attribute::Pace]);
        hm.insert(Roles::InsideForward,vec![Attribute::Shooting,Attribute::Dribbling,Attribute::Decision,Attribute::Movement,Attribute::Pace,Attribute::Passing]);
        hm.insert(Roles::Poacher,vec![Attribute::Movement,Attribute::Pace,Attribute::Technique,Attribute::Shooting,Attribute::Decision]);
        hm.insert(Roles::TargetForward,vec![Attribute::Strength,Attribute::Aero,Attribute::Movement,Attribute::Shooting]);
        hm.insert(Roles::DeepLyingForward,vec![Attribute::Movement,Attribute::Decision,Attribute::Creative,Attribute::Passing,Attribute::Shooting]);
        hm.insert(Roles::AdvanceForward, vec![Attribute::Movement,Attribute::Decision,Attribute::Shooting,Attribute::Pace]);
        hm.insert(Roles::CompleteForward,vec![Attribute::Movement,Attribute::Decision,Attribute::Creative,Attribute::Passing,Attribute::Shooting,Attribute::Strength,Attribute::Aero]);
        hm.insert(Roles::PressingForward,vec![Attribute::Movement,Attribute::Decision,Attribute::Strength,Attribute::Tackle,Attribute::Teamwork]);
        hm

    }
}
