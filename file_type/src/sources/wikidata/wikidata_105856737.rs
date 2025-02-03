use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856737: FileFormat = FileFormat {
    id: 105_856_737,
    source_type: SourceType::Wikidata,
    name: "Xilinx User Constraints File",
    extensions: &["ucf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
