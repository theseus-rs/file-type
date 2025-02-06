use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349963: FileFormat = FileFormat {
    id: 27_349_963,
    source_type: SourceType::Wikidata,
    name: "TOPSAR C-Band VV Data",
    extensions: &["vvi2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
