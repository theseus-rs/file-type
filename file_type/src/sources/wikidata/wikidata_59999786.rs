use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59999786: FileFormat = FileFormat {
    id: 59_999_786,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Lock File",
    extensions: &["lck"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
