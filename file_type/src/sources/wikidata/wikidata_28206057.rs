use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206057: FileFormat = FileFormat {
    id: 28_206_057,
    source_type: SourceType::Wikidata,
    name: "ERDAS IMAGINE Gray-scale Bitmap Image",
    extensions: &["gis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
