use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206057: FileFormat = FileFormat {
    id: 28_206_057,
    source_type: SourceType::Wikidata,
    name: "ERDAS IMAGINE Gray-scale Bitmap Image",
    extensions: &["gis"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
