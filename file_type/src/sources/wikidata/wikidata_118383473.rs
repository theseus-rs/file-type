use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118383473: FileFormat = FileFormat {
    id: 118_383_473,
    source_type: SourceType::Wikidata,
    name: "Album Book file",
    extensions: &["opf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
