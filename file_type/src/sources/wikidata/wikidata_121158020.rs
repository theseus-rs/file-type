use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121158020: FileFormat = FileFormat {
    id: 121_158_020,
    source_type: SourceType::Wikidata,
    name: "Letter file",
    extensions: &["rtf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
