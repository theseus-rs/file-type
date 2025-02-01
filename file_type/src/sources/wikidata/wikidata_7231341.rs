use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7231341: FileFormat = FileFormat {
    id: 7_231_341,
    puid: "wikidata/7231341",
    name: "Portable Database Image",
    extensions: &["pdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
