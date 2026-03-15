
use phf::{Map, phf_map};

use super::GDObject;

/// Maps object IDs to their default GD object strings (as the editor would produce them).
/// Sourced from gmdkit.
/// Ids not in defaults are handled by the fallback.
pub static OBJECT_DEFAULTS: Map<i32, &'static str> = phf_map! {
    // sourced from conversation with Xtreme / gmdkit
    29i32 => "1,29,2,0,3,0,36,1,7,255,8,255,9,255,10,0.5,35,1,23,1000;",
    30i32 => "1,30,2,0,3,0,36,1,7,255,8,255,9,255,10,0.5,35,1,23,1001;",
    31i32 => "1,31,2,0,3,0,36,1,kA2,0,kA3,1,kA8,1,kA4,0,kA9,1,kA10,1,kA22,1,kA23,1,kA24,1,kA27,1,kA40,1,kA41,1,kA42,1,kA28,1,kA29,1,kA31,1,kA32,1,kA36,0,kA43,1,kA44,0,kA45,1,kA46,1,kA33,1,kA34,1,kA35,1,kA37,1,kA38,1,kA39,1,kA19,1,kA26,0,kA20,1,kA21,1,kA11,1;",
    // paste remaining entries from gmdkit here
};

/// Returns the default [`GDObject`] for the given object ID.
/// If the ID has a known entry in [`OBJECT_DEFAULTS`], it is parsed from that string.
/// Otherwise a minimal default string `"1,<id>,2,0,3,0;"` is used as the fallback.
pub fn default_object(id: i32) -> GDObject {
    let s: String;
    let default_str: &str = match OBJECT_DEFAULTS.get(&id) {
        Some(s) => s,
        None => {
            s = format!("1,{id},2,0,3,0;");
            &s
        }
    };
    GDObject::parse_str(default_str)
}
