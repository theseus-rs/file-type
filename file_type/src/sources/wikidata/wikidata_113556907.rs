use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113556907: FileFormat = FileFormat {
    id: 113_556_907,
    puid: "wikidata/113556907",
    name: "Duplicator CD Image File",
    extensions: &["tao"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
