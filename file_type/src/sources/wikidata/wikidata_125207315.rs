use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125207315: FileFormat = FileFormat {
    id: 125_207_315,
    puid: "wikidata/125207315",
    name: "VYM part",
    extensions: &["vyp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
