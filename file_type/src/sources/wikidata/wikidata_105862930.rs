use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862930: FileFormat = FileFormat {
    id: 105_862_930,
    source_type: SourceType::Wikidata,
    name: "Aleph One Marathon Markup Language",
    extensions: &["mml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
