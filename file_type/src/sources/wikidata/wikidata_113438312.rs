use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113438312: FileFormat = FileFormat {
    id: 113_438_312,
    puid: "wikidata/113438312",
    name: "EndNote Compressed Library X - X9",
    extensions: &["enlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
