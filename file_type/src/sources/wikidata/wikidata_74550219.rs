use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74550219: FileFormat = FileFormat {
    id: 74_550_219,
    source_type: SourceType::Wikidata,
    name: "Micrografx clipart index",
    extensions: &["sbj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
