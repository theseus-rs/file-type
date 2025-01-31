use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169619: FileFormat = FileFormat {
    id: 122_169_619,
    puid: "wikidata/122169619",
    name: "Task Container",
    extensions: &["rtc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
