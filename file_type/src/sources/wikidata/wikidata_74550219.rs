use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74550219: FileFormat = FileFormat {
    id: 74_550_219,
    source_type: SourceType::Wikidata,
    name: "Micrografx clipart index",
    extensions: &["sbj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
