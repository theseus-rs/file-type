use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124856858: FileFormat = FileFormat {
    id: 124_856_858,
    source_type: SourceType::Wikidata,
    name: "OpenFOAM Mesh file",
    extensions: &["msh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
