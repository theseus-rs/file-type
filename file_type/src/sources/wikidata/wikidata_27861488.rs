use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27861488: FileFormat = FileFormat {
    id: 27_861_488,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 21",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
