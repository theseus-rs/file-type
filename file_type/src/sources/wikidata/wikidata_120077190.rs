use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120077190: FileFormat = FileFormat {
    id: 120_077_190,
    source_type: SourceType::Wikidata,
    name: "ElectricImage IMAGE",
    extensions: &["ei", "img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
