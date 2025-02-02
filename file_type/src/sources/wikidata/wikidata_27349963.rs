use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27349963: FileFormat = FileFormat {
    id: 27_349_963,
    source_type: SourceType::Wikidata,
    name: "TOPSAR C-Band VV Data",
    extensions: &["vvi2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
