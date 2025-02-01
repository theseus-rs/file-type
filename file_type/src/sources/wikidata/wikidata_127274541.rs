use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127274541: FileFormat = FileFormat {
    id: 127_274_541,
    puid: "wikidata/127274541",
    name: "Pro/ENGINEER Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
