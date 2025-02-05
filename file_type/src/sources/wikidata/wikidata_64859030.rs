use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64859030: FileFormat = FileFormat {
    id: 64_859_030,
    source_type: SourceType::Wikidata,
    name: "Family Tree Maker for Windows file format",
    extensions: &["ftw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
