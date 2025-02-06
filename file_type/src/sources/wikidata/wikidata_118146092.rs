use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118146092: FileFormat = FileFormat {
    id: 118_146_092,
    source_type: SourceType::Wikidata,
    name: "Edge-coupled symmetric file",
    extensions: &["tl2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
