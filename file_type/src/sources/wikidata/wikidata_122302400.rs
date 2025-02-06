use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122302400: FileFormat = FileFormat {
    id: 122_302_400,
    source_type: SourceType::Wikidata,
    name: "HLD File",
    extensions: &["hld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
