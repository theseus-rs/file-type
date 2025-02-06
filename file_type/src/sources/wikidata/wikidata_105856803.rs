use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856803: FileFormat = FileFormat {
    id: 105_856_803,
    source_type: SourceType::Wikidata,
    name: "Ruby Gem::Specification",
    extensions: &["gemspec"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
