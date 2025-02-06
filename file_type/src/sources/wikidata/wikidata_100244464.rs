use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100244464: FileFormat = FileFormat {
    id: 100_244_464,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Letter",
    extensions: &["lt", "ltt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
