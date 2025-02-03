use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3077345: FileFormat = FileFormat {
    id: 3_077_345,
    source_type: SourceType::Wikidata,
    name: "Polygon File Format",
    extensions: &["ply"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
