use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_213: FileType = FileType {
    file_format: &FileFormat {
        id: 213,
        source_type: SourceType::Linguist,
        name: "Lua",
        extensions: &[
            "fcgi", "lua", "nse", "p8", "pd_lua", "rbxs", "rockspec", "wlua",
        ],
        media_types: &["text/x-lua"],
        signatures: &[],
        related_formats: &[],
    },
};
