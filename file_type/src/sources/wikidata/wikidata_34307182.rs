use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34307182: FileFormat = FileFormat {
    id: 34_307_182,
    source_type: SourceType::Wikidata,
    name: "ScreenWriter II",
    extensions: &["text"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
