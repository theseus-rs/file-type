use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2127640: FileFormat = FileFormat {
    id: 2_127_640,
    source_type: SourceType::Wikidata,
    name: "Railway Markup Language",
    extensions: &["railml", "railmlx"],
    media_types: &["application/xml", "application/zip", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
