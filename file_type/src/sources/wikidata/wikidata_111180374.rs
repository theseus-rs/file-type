use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111180374: FileFormat = FileFormat {
    id: 111_180_374,
    puid: "wikidata/111180374",
    name: "PressWriter File",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
