use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64859082: FileFormat = FileFormat {
    id: 64_859_082,
    source_type: SourceType::Wikidata,
    name: "Family Tree Maker for DOS file format",
    extensions: &["ftm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
