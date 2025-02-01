use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111511710: FileFormat = FileFormat {
    id: 111_511_710,
    puid: "wikidata/111511710",
    name: "TGIF File Format",
    extensions: &["obj", "tgif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
