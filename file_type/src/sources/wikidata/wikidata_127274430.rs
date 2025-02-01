use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127274430: FileFormat = FileFormat {
    id: 127_274_430,
    puid: "wikidata/127274430",
    name: "NX Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
