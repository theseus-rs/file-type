use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113162258: FileFormat = FileFormat {
    id: 113_162_258,
    source_type: SourceType::Wikidata,
    name: "MyMailManager File",
    extensions: &["mml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
