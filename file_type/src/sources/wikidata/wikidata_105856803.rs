use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856803: FileFormat = FileFormat {
    id: 105_856_803,
    puid: "wikidata/105856803",
    name: "Ruby Gem::Specification",
    extensions: &["gemspec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
