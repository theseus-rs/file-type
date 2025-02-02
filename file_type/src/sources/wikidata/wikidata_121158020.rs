use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121158020: FileFormat = FileFormat {
    id: 121_158_020,
    source_type: SourceType::Wikidata,
    name: "Letter file",
    extensions: &["rtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
