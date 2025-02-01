use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131153783: FileFormat = FileFormat {
    id: 131_153_783,
    puid: "wikidata/131153783",
    name: "squid configuration file format",
    extensions: &["squid.conf"],
    media_types: &["text/x-squidconf"],
    internal_signatures: &[],
    related_formats: &[],
};
