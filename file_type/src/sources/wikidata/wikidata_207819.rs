use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_207819: FileFormat = FileFormat {
    id: 207_819,
    source_type: SourceType::Wikidata,
    name: "Standard Generalized Markup Language",
    extensions: &["sgml"],
    media_types: &["application/sgml", "text/sgml"],
    signatures: &[],
    related_formats: &[],
};
