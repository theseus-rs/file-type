use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967146: FileFormat = FileFormat {
    id: 27_967_146,
    source_type: SourceType::Wikidata,
    name: "Eureka Packer module",
    extensions: &["eu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
