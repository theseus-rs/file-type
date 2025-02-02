use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126960642: FileFormat = FileFormat {
    id: 126_960_642,
    source_type: SourceType::Wikidata,
    name: "SystemVerilog Source Code File",
    extensions: &["sv"],
    media_types: &["text/x-systemverilog"],
    internal_signatures: &[],
    related_formats: &[],
};
