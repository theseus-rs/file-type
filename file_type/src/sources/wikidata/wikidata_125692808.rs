use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125692808: FileFormat = FileFormat {
    id: 125_692_808,
    source_type: SourceType::Wikidata,
    name: "Pocket Excel Format",
    extensions: &["pxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
