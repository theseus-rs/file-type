use crate::format::FileFormat;

pub(crate) const LINGUIST_387: FileFormat = FileFormat {
    id: 387,
    puid: "linguist/387",
    name: "Verilog",
    extensions: &["v", "veo"],
    media_types: &["text/x-verilog"],
    internal_signatures: &[],
    related_formats: &[],
};
