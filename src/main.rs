use uuid::Uuid;
// use std::str::FromStr;
use std::marker;

// Transmission restriction 
// Transmission M can only move to transmition M+1 if Verbatim_Readback(Transmission M) has completed.
// TypeStates

// Idle (no ongoing mission)
struct Idle;

// Transmission: 1
// + Element: (1) Identification
// + Element: (2) Warning Order
struct T1E1;
struct T1E2;

// Transmission: 2
// +   Element: (3) Target Location
struct T2E3;

// Transmission: 3	
// +   Element: (4) Target Description
// +   Element: (5) Method of Engagement
// +   Element: (6) Method of Control
struct T3E4;
struct T3E5;
struct T3E6;

/// The Wraith is responsible for tracking all the details of the CFF procedure
/// and thus represents a carbon copy of communications between FOs and the FDC.
/// The Wraith can be used toreplay all the steps of the CFF in chronological order.
/// Every time a CFF is concluded, the details as logged by the Wraith are hashed
/// and stored. This log can be validated by running a Wraith::validate(id: uuid)
/// wich will return true if the logged details of each CFF corresponds to its stored hash value. 
struct Wraith {
    log: Vec<String>,
    // Might add a unique name to a created wraith - but have to consider
    // possiblity of callsign being same as wraith name.
    // name: Thyner, Ductur, Fearid, Drecon, Shent, Astel, Morrid
    // valid: bool,
}

impl Wraith {
    pub fn new() -> Self {
        Wraith {
            log: Vec::new(),
        }
    }

    /*
    // Looking to persist each communication to an sqlite db
    // The question is if there should be one database that multiple wraiths
    // can write to - or if each wraith should create its own database when it's
    // spun up.
    pub fn persist() {
        SQL CREATE TABLE cff;
    }
    */
    pub fn validate() -> bool {
        unimplemented!()
    }
}

/// # Type of Mission
/// There are six (6) types of fire missions that the FO can request.  
/// 
/// ## ADJUST FIRE
/// Indirect fire with the intent of walking rounds onto the target using `bracketing`
/// a process where repeating, increasingly more accurate single shots, are used
/// to zero in on the intended target.  
/// When a shot lands on target, or within 50 meters of the target, the FO requests 
/// `FIRE FOR EFFECT`, optionally with a final adjustment.
///  
/// ## FIRE FOR EFFECT - 	
/// ## SUPPRESSION - 
/// ## IMMEDIATE SUPPRESSION
/// ## SMOKE
/// ## ILLUMINATION
#[derive(Debug)]
enum TypeOfMission {
    AdjustFire,
    FireForEffect,
    Suppression,
    ImmediateSuppression,
    Smoke,
    Illumination,
}

/// The FO can choose from three (3) methods when he wants to communicating the 
/// location of a target to the FDC. The *default* method is `MethodOfTargetLocation::Grid`,
/// which means, if the FO does not specify a method of target location, the FDC
/// will assume it's Grid.
enum MethodOfTargetLocation {
    Grid,
    Polar,
    ShiftFromKnownPoint,
}

/// CFF (Call For Fire) represents a single fire mission communication procedure
/// between an FO (Forward Observer) and the FDC (Fire Direction Center).
struct CFF<State> {
    cff_id: String,
    call_sign: String,
    _state: marker::PhantomData<State>
}

impl CFF<Idle> {
    // Constructor used to initate a fire mission.
    pub fn new(callsign: String) -> CFF<T1E1> {
        let new_cff_id: String = Uuid::new_v4().to_string().chars().take(8).collect();
        println!("Call For Fire mission with id {} initiated by caller {}.", new_cff_id, callsign);
        // TODO: Send this to the Wraith in order to log each element.
        CFF {
            cff_id: new_cff_id,
            call_sign: callsign, 
            _state: marker::PhantomData,
        }
    }
}

/// The CFF is initiated with a verbal handshake between the FO and the FDC.
/// The FO will first identify the call sign of the FDC, and then he will identify
/// himself the FDC with his own FO call sign: "Sledgehammer, this is Merlin 1."
impl CFF<T1E1> {
    pub fn identification(self, addressee: String) -> CFF<T1E2> {
        println!("LOG: {}, this is {}.", addressee, self.call_sign);
        CFF {
            cff_id: self.cff_id,
            call_sign : self.call_sign,
            _state: marker::PhantomData,
        }
    }
}

/// The second element of the first transmission of the CFF is the `WARNORD` 
/// (Warning Order) and it should follow immediately after the 
/// CFF Transmission 1 Element (1) Identification.
/// 
/// This element is used by the FO to instruct the FDC what type of fire mission
/// he's requesting. There are six (6) mission types, and the FO can only choose
/// one of these.
impl CFF<T1E2> {
    /* 
    pub fn warning_order(self, mission: TypeOfMission, method: Option<MethodOfTargetLocation>) -> CFF<T1E2> {
        match method {
            Some(x) => { println!("Method of Target Location is: {}", x); },
            None => { println!("Method of Target Location is: Grid"); }
            }
    */        
    fn warning_order(self, mission: TypeOfMission) -> CFF<T1E2> {
        println!("LOG: Mission type {:?},", mission);
        CFF {
            cff_id: self.cff_id,
            call_sign : self.call_sign,
            _state: marker::PhantomData,
        }
    }
}

fn main() {
    // The wraith will be in charge of 
    // https://namesbeast.com/wraith-names/
    let wraith = Wraith::new();
    
    let cff01 = CFF::new(String::from("Merlin 1"));         
    // State: CFF<Idle>

    let cff01 = cff01.identification(String::from("Sledgehammer"));
    // New state: CFF<T1E1>
    
    let cff01 = cff01.warning_order(TypeOfMission::Smoke);
    // New state: CFF<T1E2>
}