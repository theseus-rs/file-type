use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62561230: FileFormat = FileFormat {
    id: 62_561_230,
    source_type: SourceType::Wikidata,
    name: "Corel Presentation, version 3",
    extensions: &["shw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
