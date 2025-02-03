use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113162258: FileFormat = FileFormat {
    id: 113_162_258,
    source_type: SourceType::Wikidata,
    name: "MyMailManager File",
    extensions: &["mml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
