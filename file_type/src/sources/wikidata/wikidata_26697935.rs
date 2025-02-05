use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26697935: FileFormat = FileFormat {
    id: 26_697_935,
    source_type: SourceType::Wikidata,
    name: "PHP script",
    extensions: &["php"],
    media_types: &["text/x-php"],
    signatures: &[],
    related_formats: &[],
};
