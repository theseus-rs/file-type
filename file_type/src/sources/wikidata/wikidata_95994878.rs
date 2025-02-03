use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95994878: FileFormat = FileFormat {
    id: 95_994_878,
    source_type: SourceType::Wikidata,
    name: "Canadian digital elevation data format",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
