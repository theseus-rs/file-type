use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114891689: FileFormat = FileFormat {
    id: 114_891_689,
    source_type: SourceType::Wikidata,
    name: "Quicken Rental Property Manager File",
    extensions: &["qrp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
