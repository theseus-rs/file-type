use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47196445: FileFormat = FileFormat {
    id: 47_196_445,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Database file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
