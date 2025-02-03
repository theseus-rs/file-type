use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131287731: FileFormat = FileFormat {
    id: 131_287_731,
    source_type: SourceType::Wikidata,
    name: "Tea Template file format",
    extensions: &["tea"],
    media_types: &["text/x-tea"],
    internal_signatures: &[],
    related_formats: &[],
};
