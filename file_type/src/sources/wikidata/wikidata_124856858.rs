use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124856858: FileFormat = FileFormat {
    id: 124_856_858,
    source_type: SourceType::Wikidata,
    name: "OpenFOAM Mesh file",
    extensions: &["msh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
