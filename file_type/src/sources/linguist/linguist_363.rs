use crate::format::FileFormat;

pub(crate) const LINGUIST_363: FileFormat = FileFormat {
    id: 363,
    puid: "linguist/363",
    name: "SystemVerilog",
    extensions: &["sv", "svh", "vh"],
    media_types: &["text/x-systemverilog"],
    internal_signatures: &[],
    related_formats: &[],
};
