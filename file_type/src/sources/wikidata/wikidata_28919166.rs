use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919166: FileFormat = FileFormat {
    id: 28_919_166,
    source_type: SourceType::Wikidata,
    name: "GHS Geometry",
    extensions: &["gf", "gft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
