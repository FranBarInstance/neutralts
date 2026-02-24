use serde_json::json;
use crate::{
    constants::*,
    utils::*,
    shared::Shared,
    bif::Bif,
    utils::extract_blocks
};

pub(crate) struct BlockInherit {
    pub(crate) indir: String,
    pub(crate) last_bif_out: bool,
    pub(crate) last_coalesce_out: bool,
    pub(crate) block_count: u64, // u64 is default type in Value nums
    pub(crate) bif_count: u64,   // u64 is default type in Value nums
    pub(crate) alias: String,
    pub(crate) current_file: String,
    pub(crate) current_dir: String,
    pub(crate) include_files: Vec<String>,
    pub(crate) locale_files: Vec<String>,
    pub(crate) data_files: Vec<String>,
    pub(crate) in_cache: bool,
    pub(crate) in_only: bool,
}

impl Clone for BlockInherit {
    fn clone(&self) -> Self {
        BlockInherit {
            indir: self.indir.clone(),
            last_bif_out: self.last_bif_out,
            last_coalesce_out: self.last_coalesce_out,
            block_count: self.block_count,
            bif_count: self.bif_count,
            alias: self.alias.clone(),
            current_file: self.current_file.clone(),
            current_dir: self.current_dir.clone(),
            include_files: self.include_files.clone(),
            locale_files: self.locale_files.clone(),
            data_files: self.data_files.clone(),
            in_cache: self.in_cache,
            in_only: self.in_only,
        }
    }
}

impl BlockInherit {
    pub(crate) fn new() -> Self {
        BlockInherit {
            indir: "block_0".to_string(),
            last_bif_out: false,
            last_coalesce_out: false,
            block_count: 0,
            bif_count: 0,
            alias: String::new(),
            current_file: String::new(),
            current_dir: String::new(),
            include_files: Vec::new(),
            locale_files: Vec::new(),
            data_files: Vec::new(),
            in_cache: false,
            in_only: false,
        }
    }

    // Create version of data for inheritance at the block level.
    // For performance reasons, instead of inheriting the complete cloned schema,
    // we inherit a reference to the data in the root schema.
    // Therefore, this function should be called before creating data
    // that needs to be inherited to obtain the reference to the storage.
    pub(crate) fn create_block_schema(&mut self, shared: &mut Shared) -> String {
        let prev_id = self.indir.clone();
        let block_id;

        // If this function is called before creating the first block.
        // It may be necessary to initialize values.
        // The first block is not 0, is 1.
        if self.block_count < 1 {
            block_id = "block_1".to_string();
        } else {
            block_id = "block_".to_string() + self.block_count.to_string().as_str();
        }

        // It can be called several times from the same level, in which case
        // it does not need to be cloned again.
        if prev_id != block_id {
            shared.schema["__indir"][&block_id] = shared.schema["__indir"][&prev_id].clone();
        }

        self.indir = block_id.clone();

        block_id
    }
}


pub(crate) struct BlockParser<'a> {
    shared: &'a mut Shared,
    inherit: BlockInherit,
    _none: &'a str,
}

impl Drop for BlockParser<'_> {
    fn drop(&mut self) {
        // release memory
        let block_id = "block_".to_string() + self.inherit.block_count.to_string().as_str();

        // The first main block cannot be deleted
        if block_id != "block_1" {
            if block_id == self.inherit.indir && is_defined_key(&self.shared.schema["__indir"], &block_id) {
                self.shared.schema["__indir"][&block_id] = json!({});
            }
        }
    }
}

impl<'a> BlockParser<'a> {
    pub(crate) fn new(shared: &'a mut Shared, inherit: &BlockInherit) -> Self {
        let mut inherit = inherit.clone();
        inherit.block_count += 1;

        BlockParser {
            shared,
            inherit,
            _none: "",
        }
    }

    pub(crate) fn update_indir(&mut self, indir: &String) {
        self.shared.schema["__indir"][indir] =
            self.shared.schema["__indir"][&self.inherit.indir].clone();
    }

    pub(crate) fn parse(&mut self, raw_source: &'a str, only: &str) -> String {
        let blocks = match extract_blocks(raw_source) {
            Ok(b) => b,
            Err(p) => {
                self.handle_unmatched_block(p);
                return EMPTY_STRING;
            }
        };

        self.parse_with_blocks(raw_source, &blocks, only)
    }

    fn handle_unmatched_block(&mut self, p: usize) {
        self.shared.status_code = "500".to_string();
        self.shared.status_param = format!("Unmatched block at position {}", p);
        eprintln!("Unmatched block at position {}", p);

        if let Some(text) = STATUS_CODES.get(self.shared.status_code.as_str()) {
            self.shared.status_text = text.to_string();
        } else {
            self.shared.status_text = EMPTY_STRING;
        }
    }

    pub(crate) fn parse_with_blocks(
        &mut self,
        raw_source: &'a str,
        blocks: &[(usize, usize)],
        only: &str,
    ) -> String {
        let mut prev_end = 0;
        let mut out = String::new();
        for (start, end) in blocks {
            let start = *start;
            let end = *end;
            let is_comment = raw_source[start..end].starts_with(BIF_COMMENT_OPEN);
            let is_short_circuit_coalesce =
                self.inherit.last_coalesce_out && self.inherit.alias == "coalesce";

            if self.shared.exit {
                return out.clone();
            }

            if prev_end < start {
                out += &raw_source[prev_end..start];
            }

            if !is_comment && !is_short_circuit_coalesce {
                let mut bif =
                    Bif::new(&raw_source[start..end], self.shared, &mut self.inherit, only);
                out += &bif.parse();
            }

            prev_end = end;
        }
        out += &raw_source[prev_end..];

        out.trim().to_string()
    }
}
