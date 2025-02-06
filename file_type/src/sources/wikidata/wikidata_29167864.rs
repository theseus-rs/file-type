use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167864: FileFormat = FileFormat {
    id: 29_167_864,
    source_type: SourceType::Wikidata,
    name: "Pittsburgh Supercomputing Center 3D Metafile",
    extensions: &["p3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
