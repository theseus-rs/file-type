use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967388: FileFormat = FileFormat {
    id: 27_967_388,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker instrument",
    extensions: &["ins"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
