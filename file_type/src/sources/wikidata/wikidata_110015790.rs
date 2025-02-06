use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110015790: FileFormat = FileFormat {
    id: 110_015_790,
    source_type: SourceType::Wikidata,
    name: "OrCAD Layout File",
    extensions: &["max"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
