use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29151874: FileFormat = FileFormat {
    id: 29_151_874,
    source_type: SourceType::Wikidata,
    name: "QRT Ray Tracer scene description",
    extensions: &["qrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
