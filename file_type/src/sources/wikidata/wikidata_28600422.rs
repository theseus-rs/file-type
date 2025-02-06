use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600422: FileFormat = FileFormat {
    id: 28_600_422,
    source_type: SourceType::Wikidata,
    name: "4bottle",
    extensions: &["4q"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
