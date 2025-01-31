use crate::format::FileFormat;

pub(crate) const LINGUIST_385: FileFormat = FileFormat {
    id: 385,
    puid: "linguist/385",
    name: "VHDL",
    extensions: &["vhd", "vhdl", "vhf", "vhi", "vho", "vhs", "vht", "vhw"],
    media_types: &["text/x-vhdl"],
    internal_signatures: &[],
    related_formats: &[],
};
