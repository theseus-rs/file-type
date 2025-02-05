use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47203283: FileFormat = FileFormat {
    id: 47_203_283,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Presentation file format",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
