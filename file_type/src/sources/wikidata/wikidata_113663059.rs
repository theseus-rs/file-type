use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113663059: FileFormat = FileFormat {
    id: 113_663_059,
    puid: "wikidata/113663059",
    name: "Coordinate 3D",
    extensions: &["c3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
