use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861483: FileFormat = FileFormat {
    id: 27_861_483,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 14",
    extensions: &["xrns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
