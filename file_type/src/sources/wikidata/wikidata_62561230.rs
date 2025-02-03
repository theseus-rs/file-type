use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62561230: FileFormat = FileFormat {
    id: 62_561_230,
    source_type: SourceType::Wikidata,
    name: "Corel Presentation, version 3",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
