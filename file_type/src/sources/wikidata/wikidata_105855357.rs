use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855357: FileFormat = FileFormat {
    id: 105_855_357,
    puid: "wikidata/105855357",
    name: "Flatpack Reference (with rem)",
    extensions: &["flatpakref"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
