use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109944694: FileFormat = FileFormat {
    id: 109_944_694,
    source_type: SourceType::Wikidata,
    name: "Archives file format",
    extensions: &["arv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
