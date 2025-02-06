use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857934: FileFormat = FileFormat {
    id: 105_857_934,
    source_type: SourceType::Wikidata,
    name: "ISDOC electronic invoice",
    extensions: &["isdoc"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
