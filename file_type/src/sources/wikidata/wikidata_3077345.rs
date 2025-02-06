use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3077345: FileFormat = FileFormat {
    id: 3_077_345,
    source_type: SourceType::Wikidata,
    name: "Polygon File Format",
    extensions: &["ply"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
