use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967146: FileFormat = FileFormat {
    id: 27_967_146,
    source_type: SourceType::Wikidata,
    name: "Eureka Packer module",
    extensions: &["eu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
