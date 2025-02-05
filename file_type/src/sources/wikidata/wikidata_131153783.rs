use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131153783: FileFormat = FileFormat {
    id: 131_153_783,
    source_type: SourceType::Wikidata,
    name: "squid configuration file format",
    extensions: &["squid.conf"],
    media_types: &["text/x-squidconf"],
    signatures: &[],
    related_formats: &[],
};
