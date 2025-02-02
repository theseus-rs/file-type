use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967388: FileFormat = FileFormat {
    id: 27_967_388,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
