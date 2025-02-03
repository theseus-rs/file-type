use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29168491: FileFormat = FileFormat {
    id: 29_168_491,
    source_type: SourceType::Wikidata,
    name: "InfluxDB TSM file",
    extensions: &["tsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
