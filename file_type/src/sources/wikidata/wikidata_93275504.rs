use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_93275504: FileFormat = FileFormat {
    id: 93_275_504,
    source_type: SourceType::Wikidata,
    name: "Procreate",
    extensions: &["procreate"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
