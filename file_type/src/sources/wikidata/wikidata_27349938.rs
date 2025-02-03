use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27349938: FileFormat = FileFormat {
    id: 27_349_938,
    source_type: SourceType::Wikidata,
    name: "TOPSAR Digital Elevation Model",
    extensions: &["demi2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
