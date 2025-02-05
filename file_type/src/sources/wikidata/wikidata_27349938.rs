use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349938: FileFormat = FileFormat {
    id: 27_349_938,
    source_type: SourceType::Wikidata,
    name: "TOPSAR Digital Elevation Model",
    extensions: &["demi2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
