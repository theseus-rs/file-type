use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25103897: FileFormat = FileFormat {
    id: 25_103_897,
    source_type: SourceType::Wikidata,
    name: "Dynamic Text Document",
    extensions: &["dtxt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
