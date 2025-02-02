use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975670: FileFormat = FileFormat {
    id: 28_975_670,
    source_type: SourceType::Wikidata,
    name: "Cyberspace Description Format",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
