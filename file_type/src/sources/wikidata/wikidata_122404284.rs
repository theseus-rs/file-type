use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122404284: FileFormat = FileFormat {
    id: 122_404_284,
    puid: "wikidata/122404284",
    name: "Pilot Resource File",
    extensions: &["plr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
