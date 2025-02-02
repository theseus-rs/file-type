use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109996883: FileFormat = FileFormat {
    id: 109_996_883,
    source_type: SourceType::Wikidata,
    name: "Primavera P6 Project Management XER File",
    extensions: &["xer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
