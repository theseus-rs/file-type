use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125871478: FileFormat = FileFormat {
    id: 125_871_478,
    source_type: SourceType::Wikidata,
    name: "PechaMaker Format",
    extensions: &["pxp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
