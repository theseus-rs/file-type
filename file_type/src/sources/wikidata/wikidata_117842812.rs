use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117842812: FileFormat = FileFormat {
    id: 117_842_812,
    source_type: SourceType::Wikidata,
    name: "EDMICS 5",
    extensions: &["ed5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
