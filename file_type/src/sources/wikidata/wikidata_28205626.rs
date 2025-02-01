use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205626: FileFormat = FileFormat {
    id: 28_205_626,
    puid: "wikidata/28205626",
    name: "Sun icon",
    extensions: &["ico", "icon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
