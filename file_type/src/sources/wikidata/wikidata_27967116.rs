use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967116: FileFormat = FileFormat {
    id: 27_967_116,
    source_type: SourceType::Wikidata,
    name: "ASC Sound Master module",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
