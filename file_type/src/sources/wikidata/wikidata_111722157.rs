use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111722157: FileFormat = FileFormat {
    id: 111_722_157,
    puid: "wikidata/111722157",
    name: "WiDE Project File",
    extensions: &["wpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
