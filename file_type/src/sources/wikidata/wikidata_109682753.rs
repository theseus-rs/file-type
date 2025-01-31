use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109682753: FileFormat = FileFormat {
    id: 109_682_753,
    puid: "wikidata/109682753",
    name: "WinAce Archive",
    extensions: &["rar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
