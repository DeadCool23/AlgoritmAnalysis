use std::vec;
use lazy_static::lazy_static;

use super::AncientWorld;

lazy_static!{
    pub static ref CANAAN: AncientWorld = AncientWorld {
        cities_count: 5,
        cities_names: vec![
            "Jerusalem".to_string(),
            "Jericho".to_string(),
            "Gaza".to_string(),
            "Hebron".to_string(),
            "Ashkelon".to_string(),
        ],
        cities_roads: vec![
            vec![0, 27, 77, 30, 60],
            vec![27, 0, 90, 40, 87],
            vec![77, 90, 0, 50, 20],
            vec![30, 40, 50, 0, 45],
            vec![60, 87, 20, 45, 0],
        ],
    };
    
    pub static ref MESOPOTAMIA: AncientWorld = AncientWorld {
        cities_count: 10,
        cities_names: vec![
            "Babylon".to_string(),
            "Uruk".to_string(),
            "Ur".to_string(),
            "Nippur".to_string(),
            "Lagash".to_string(),
            "Nineveh".to_string(),
            "Ashur".to_string(),
            "Eridu".to_string(),
            "Sippar".to_string(),
            "Eshnunna".to_string(),
        ],
        cities_roads: vec![
            vec![0, 200, 300, 100, 250, 450, 500, 400, 150, 120], 
            vec![200, 0, 100, 150, 300, 500, 550, 300, 300, 200], 
            vec![300, 100, 0, 250, 150, 600, 650, 200, 350, 300], 
            vec![100, 150, 250, 0, 100, 500, 550, 300, 150, 170], 
            vec![250, 300, 150, 100, 0, 600, 650, 200, 300, 300], 
            vec![450, 500, 600, 500, 600, 0, 150, 700, 350, 400], 
            vec![500, 550, 650, 550, 650, 150, 0, 750, 400, 450], 
            vec![400, 300, 200, 300, 200, 700, 750, 0, 350, 300], 
            vec![150, 300, 350, 150, 300, 350, 400, 350, 0, 100], 
            vec![120, 200, 300, 170, 300, 400, 450, 300, 100, 0], 
        ],
    };

    pub static ref ANCIENT_ROME: AncientWorld = AncientWorld {
        cities_count: 15,
        cities_names: vec![
            "Rome".to_string(),
            "Pompeii".to_string(),
            "Carthage".to_string(),
            "Athens".to_string(),
            "Alexandria".to_string(),
            "Byzantium".to_string(),
            "Antioch".to_string(),
            "Ephesus".to_string(),
            "Lugdunum".to_string(),
            "Londinium".to_string(),
            "Massilia".to_string(),
            "Ravenna".to_string(),
            "Verona".to_string(),
            "Brundisium".to_string(),
            "Capua".to_string(),
        ],
        cities_roads: vec![
            vec![0, 240, 820, 1000, 1300, 1400, 1100, 950, 450, 1200, 500, 300, 400, 700, 180], 
            vec![240, 0, 780, 970, 1250, 1400, 1150, 900, 500, 1250, 550, 350, 450, 750, 60],   
            vec![820, 780, 0, 1500, 1500, 2000, 1700, 1550, 1300, 2100, 950, 1000, 1200, 1700, 800], 
            vec![1000, 970, 1500, 0, 600, 400, 700, 600, 1200, 1600, 700, 800, 950, 1200, 950], 
            vec![1300, 1250, 1500, 600, 0, 400, 300, 700, 1500, 1900, 1000, 1300, 1500, 1700, 1200], 
            vec![1400, 1400, 2000, 400, 400, 0, 500, 300, 1600, 2000, 1200, 1500, 1700, 2000, 1350], 
            vec![1100, 1150, 1700, 700, 300, 500, 0, 400, 1300, 1800, 900, 1200, 1400, 1500, 1050], 
            vec![950, 900, 1550, 600, 700, 300, 400, 0, 1000, 1400, 750, 950, 1150, 1300, 850],   
            vec![450, 500, 1300, 1200, 1500, 1600, 1300, 1000, 0, 800, 450, 300, 350, 750, 350],  
            vec![1200, 1250, 2100, 1600, 1900, 2000, 1800, 1400, 800, 0, 1250, 1400, 1600, 1750, 1300], 
            vec![500, 550, 950, 700, 1000, 1200, 900, 750, 450, 1250, 0, 250, 350, 550, 400],     
            vec![300, 350, 1000, 800, 1300, 1500, 1200, 950, 300, 1400, 250, 0, 150, 500, 250],   
            vec![400, 450, 1200, 950, 1500, 1700, 1400, 1150, 350, 1600, 350, 150, 0, 400, 350],  
            vec![700, 750, 1700, 1200, 1700, 2000, 1500, 1300, 750, 1750, 550, 500, 400, 0, 700], 
            vec![180, 60, 800, 950, 1200, 1350, 1050, 850, 350, 1300, 400, 250, 350, 700, 0],     
        ],
    };    

    pub static ref MAIN_COUNTRIES: Vec<&'static AncientWorld> = vec![&CANAAN, &MESOPOTAMIA, &ANCIENT_ROME];
}
