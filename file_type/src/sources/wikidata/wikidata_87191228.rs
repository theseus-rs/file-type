use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87191228: FileFormat = FileFormat {
    id: 87_191_228,
    puid: "wikidata/87191228",
    name: "X3D 3.2",
    extensions: &["x3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
