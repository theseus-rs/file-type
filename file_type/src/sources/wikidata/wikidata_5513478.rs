use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5513478: FileFormat = FileFormat {
    id: 5_513_478,
    source_type: SourceType::Wikidata,
    name: "GIFT",
    extensions: &["gift"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
