use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111330701: FileFormat = FileFormat {
    id: 111_330_701,
    puid: "wikidata/111330701",
    name: "MadTracker 2 instruments",
    extensions: &["mti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
