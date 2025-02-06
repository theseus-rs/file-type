use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975670: FileFormat = FileFormat {
    id: 28_975_670,
    source_type: SourceType::Wikidata,
    name: "Cyberspace Description Format",
    extensions: &["cdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
