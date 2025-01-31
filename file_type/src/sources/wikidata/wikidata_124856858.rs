use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124856858: FileFormat = FileFormat {
    id: 124_856_858,
    puid: "wikidata/124856858",
    name: "OpenFOAM Mesh file",
    extensions: &["msh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
