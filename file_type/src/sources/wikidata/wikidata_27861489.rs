use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861489: FileFormat = FileFormat {
    id: 27_861_489,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 22",
    extensions: &["xrns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
