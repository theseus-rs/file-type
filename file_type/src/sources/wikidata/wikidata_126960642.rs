use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126960642: FileFormat = FileFormat {
    id: 126_960_642,
    puid: "wikidata/126960642",
    name: "SystemVerilog Source Code File",
    extensions: &["sv"],
    media_types: &["text/x-systemverilog"],
    internal_signatures: &[],
    related_formats: &[],
};
