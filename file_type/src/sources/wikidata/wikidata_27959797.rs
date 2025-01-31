use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959797: FileFormat = FileFormat {
    id: 27_959_797,
    puid: "wikidata/27959797",
    name: "Ableton Device Preset",
    extensions: &["adp", "adv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
