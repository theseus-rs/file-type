use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124671792: FileFormat = FileFormat {
    id: 124_671_792,
    puid: "wikidata/124671792",
    name: "Archive eXchange Format",
    extensions: &["axf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
