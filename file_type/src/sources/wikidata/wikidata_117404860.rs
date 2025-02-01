use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117404860: FileFormat = FileFormat {
    id: 117_404_860,
    puid: "wikidata/117404860",
    name: "VHDL Output File",
    extensions: &["vho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
