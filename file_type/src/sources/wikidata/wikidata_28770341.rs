use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770341: FileFormat = FileFormat {
    id: 28_770_341,
    puid: "wikidata/28770341",
    name: "M2k",
    extensions: &["m2k"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
