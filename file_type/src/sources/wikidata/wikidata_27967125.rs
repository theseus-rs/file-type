use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967125: FileFormat = FileFormat {
    id: 27_967_125,
    source_type: SourceType::Wikidata,
    name: "CMC",
    extensions: &["cmc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
