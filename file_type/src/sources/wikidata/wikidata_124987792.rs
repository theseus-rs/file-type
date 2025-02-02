use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124987792: FileFormat = FileFormat {
    id: 124_987_792,
    source_type: SourceType::Wikidata,
    name: "Dr.Geo document",
    extensions: &["fgeo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
