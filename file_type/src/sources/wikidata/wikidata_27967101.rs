use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967101: FileFormat = FileFormat {
    id: 27_967_101,
    source_type: SourceType::Wikidata,
    name: "Nintendo GameCube/Wii ADP",
    extensions: &["adp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
