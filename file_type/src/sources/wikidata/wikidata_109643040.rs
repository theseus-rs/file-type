use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109643040: FileFormat = FileFormat {
    id: 109_643_040,
    source_type: SourceType::Wikidata,
    name: "VJ file format",
    extensions: &["vj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
