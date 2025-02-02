use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119496056: FileFormat = FileFormat {
    id: 119_496_056,
    source_type: SourceType::Wikidata,
    name: "IBM IOCA Raw",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
