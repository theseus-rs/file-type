use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122075846: FileFormat = FileFormat {
    id: 122_075_846,
    puid: "wikidata/122075846",
    name: "Finale Lesson File",
    extensions: &["lsn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
