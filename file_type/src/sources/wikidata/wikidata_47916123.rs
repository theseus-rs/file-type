use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47916123: FileFormat = FileFormat {
    id: 47_916_123,
    source_type: SourceType::Wikidata,
    name: "Frame Vector Metafile",
    extensions: &["fmv"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
