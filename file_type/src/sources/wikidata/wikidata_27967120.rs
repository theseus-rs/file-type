use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967120: FileFormat = FileFormat {
    id: 27_967_120,
    puid: "wikidata/27967120",
    name: "Brian Postma SoundMon v1.x module",
    extensions: &["bp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
