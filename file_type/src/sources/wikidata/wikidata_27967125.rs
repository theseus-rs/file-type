use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967125: FileFormat = FileFormat {
    id: 27_967_125,
    source_type: SourceType::Wikidata,
    name: "CMC",
    extensions: &["cmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
