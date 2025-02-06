use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861488: FileFormat = FileFormat {
    id: 27_861_488,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 21",
    extensions: &["xrns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
