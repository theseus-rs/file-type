use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857934: FileFormat = FileFormat {
    id: 105_857_934,
    source_type: SourceType::Wikidata,
    name: "ISDOC electronic invoice",
    extensions: &["isdoc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
