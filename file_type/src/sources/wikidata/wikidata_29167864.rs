use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167864: FileFormat = FileFormat {
    id: 29_167_864,
    puid: "wikidata/29167864",
    name: "Pittsburgh Supercomputing Center 3D Metafile",
    extensions: &["p3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
