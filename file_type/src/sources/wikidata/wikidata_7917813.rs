use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7917813: FileFormat = FileFormat {
    id: 7_917_813,
    puid: "wikidata/7917813",
    name: "Vector Product Format",
    extensions: &["vpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
