use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979322: FileFormat = FileFormat {
    id: 27_979_322,
    puid: "wikidata/27979322",
    name: "Photoshop Curve",
    extensions: &["crv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
