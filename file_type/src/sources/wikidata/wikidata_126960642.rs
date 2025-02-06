use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126960642: FileFormat = FileFormat {
    id: 126_960_642,
    source_type: SourceType::Wikidata,
    name: "SystemVerilog Source Code File",
    extensions: &["sv"],
    media_types: &["text/x-systemverilog"],
    signatures: &[],
    related_formats: &[],
};
