use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114891689: FileFormat = FileFormat {
    id: 114_891_689,
    source_type: SourceType::Wikidata,
    name: "Quicken Rental Property Manager File",
    extensions: &["qrp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
