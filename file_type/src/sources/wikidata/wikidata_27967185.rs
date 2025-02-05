use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967185: FileFormat = FileFormat {
    id: 27_967_185,
    source_type: SourceType::Wikidata,
    name: "Fuchs Tracker",
    extensions: &["fchs", "ft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
