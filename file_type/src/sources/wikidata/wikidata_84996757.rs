use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84996757: FileFormat = FileFormat {
    id: 84_996_757,
    source_type: SourceType::Wikidata,
    name: "HP Photo Album",
    extensions: &["albm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
