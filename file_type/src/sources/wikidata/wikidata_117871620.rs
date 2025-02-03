use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117871620: FileFormat = FileFormat {
    id: 117_871_620,
    source_type: SourceType::Wikidata,
    name: "U.S. Robotics WorldPort 2496 file",
    extensions: &["wpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
