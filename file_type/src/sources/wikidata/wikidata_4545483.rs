use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4545483: FileFormat = FileFormat {
    id: 4_545_483,
    source_type: SourceType::Wikidata,
    name: "X File Format",
    extensions: &["x"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
