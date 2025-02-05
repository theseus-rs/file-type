use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64859082: FileFormat = FileFormat {
    id: 64_859_082,
    source_type: SourceType::Wikidata,
    name: "Family Tree Maker for DOS file format",
    extensions: &["ftm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
