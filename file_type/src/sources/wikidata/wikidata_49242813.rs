use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49242813: FileFormat = FileFormat {
    id: 49_242_813,
    puid: "wikidata/49242813",
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
