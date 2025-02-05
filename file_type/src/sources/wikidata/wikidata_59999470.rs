use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59999470: FileFormat = FileFormat {
    id: 59_999_470,
    source_type: SourceType::Wikidata,
    name: "ESRI Spatial Index File",
    extensions: &["sbn", "sbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
