use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117871620: FileFormat = FileFormat {
    id: 117_871_620,
    source_type: SourceType::Wikidata,
    name: "U.S. Robotics WorldPort 2496 file",
    extensions: &["wpf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
