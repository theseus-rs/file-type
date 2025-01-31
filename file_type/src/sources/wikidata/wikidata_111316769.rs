use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316769: FileFormat = FileFormat {
    id: 111_316_769,
    puid: "wikidata/111316769",
    name: "Impulse Tracker instrument",
    extensions: &["iti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
