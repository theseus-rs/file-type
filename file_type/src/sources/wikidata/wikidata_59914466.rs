use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59914466: FileFormat = FileFormat {
    id: 59_914_466,
    source_type: SourceType::Wikidata,
    name: "Deluxe Paint bitmap",
    extensions: &["lbm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
