use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87190680: FileFormat = FileFormat {
    id: 87_190_680,
    puid: "wikidata/87190680",
    name: "X3D 3.1",
    extensions: &["x3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
