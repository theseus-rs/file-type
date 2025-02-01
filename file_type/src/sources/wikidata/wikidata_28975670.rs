use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975670: FileFormat = FileFormat {
    id: 28_975_670,
    puid: "wikidata/28975670",
    name: "Cyberspace Description Format",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
