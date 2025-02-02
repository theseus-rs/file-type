use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84037847: FileFormat = FileFormat {
    id: 84_037_847,
    source_type: SourceType::Wikidata,
    name: "WARC 1.1",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[],
    related_formats: &[],
};
