use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47916123: FileFormat = FileFormat {
    id: 47_916_123,
    source_type: SourceType::Wikidata,
    name: "Frame Vector Metafile",
    extensions: &["fmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
