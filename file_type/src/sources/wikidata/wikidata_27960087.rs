use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960087: FileFormat = FileFormat {
    id: 27_960_087,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice",
    extensions: &["msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
