use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85413270: FileFormat = FileFormat {
    id: 85_413_270,
    source_type: SourceType::Wikidata,
    name: "PTGui Project File 11",
    extensions: &["pts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
