use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_213: FileFormat = FileFormat {
    id: 213,
    source_type: SourceType::Linguist,
    name: "Lua",
    extensions: &[
        "fcgi", "lua", "nse", "p8", "pd_lua", "rbxs", "rockspec", "wlua",
    ],
    media_types: &["text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};
