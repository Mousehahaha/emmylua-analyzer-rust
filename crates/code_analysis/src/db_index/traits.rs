use std::collections::HashMap;

use crate::FileId;

pub trait LuaIndex {
    fn remove(&mut self, file_id: FileId);

    fn fill_snapshot_info(&self, info: &mut HashMap<String, String>);
}
