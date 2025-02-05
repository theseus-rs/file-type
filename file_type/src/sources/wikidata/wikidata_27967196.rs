use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967196: FileFormat = FileFormat {
    id: 27_967_196,
    source_type: SourceType::Wikidata,
    name: "Impulse Tracker sample",
    extensions: &["its"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
