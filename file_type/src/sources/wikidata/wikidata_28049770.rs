use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049770: FileFormat = FileFormat {
    id: 28_049_770,
    puid: "wikidata/28049770",
    name: "DKBTrace scene description",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
