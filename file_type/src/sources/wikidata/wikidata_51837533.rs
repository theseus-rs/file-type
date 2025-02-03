use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51837533: FileFormat = FileFormat {
    id: 51_837_533,
    source_type: SourceType::Wikidata,
    name: "Visual FoxPro Database Container File",
    extensions: &["dcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
