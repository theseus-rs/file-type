use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_18653981: FileFormat = FileFormat {
    id: 18_653_981,
    source_type: SourceType::Wikidata,
    name: "Standard Delay Format",
    extensions: &["sdf", "sdo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
