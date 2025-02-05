use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118146513: FileFormat = FileFormat {
    id: 118_146_513,
    source_type: SourceType::Wikidata,
    name: "Coaxial Cable File",
    extensions: &["tl7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
