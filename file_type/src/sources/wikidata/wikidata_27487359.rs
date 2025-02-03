use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27487359: FileFormat = FileFormat {
    id: 27_487_359,
    source_type: SourceType::Wikidata,
    name: "ArcView GIS 3.x attribute index",
    extensions: &["atx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
