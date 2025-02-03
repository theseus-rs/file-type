use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120077190: FileFormat = FileFormat {
    id: 120_077_190,
    source_type: SourceType::Wikidata,
    name: "ElectricImage IMAGE",
    extensions: &["ei", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
