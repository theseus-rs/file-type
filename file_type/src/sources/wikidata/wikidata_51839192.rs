use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51839192: FileFormat = FileFormat {
    id: 51_839_192,
    source_type: SourceType::Wikidata,
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
