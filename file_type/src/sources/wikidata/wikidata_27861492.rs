use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861492: FileFormat = FileFormat {
    id: 27_861_492,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 37",
    extensions: &["xrns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
