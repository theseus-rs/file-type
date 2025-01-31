use crate::format::FileFormat;

pub(crate) const LINGUIST_213: FileFormat = FileFormat {
    id: 213,
    puid: "linguist/213",
    name: "Lua",
    extensions: &[
        "fcgi", "lua", "nse", "p8", "pd_lua", "rbxs", "rockspec", "wlua",
    ],
    media_types: &["text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};
