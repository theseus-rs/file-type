use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27861480: FileFormat = FileFormat {
    id: 27_861_480,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 9",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
