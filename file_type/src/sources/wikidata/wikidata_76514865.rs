use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76514865: FileFormat = FileFormat {
    id: 76_514_865,
    puid: "wikidata/76514865",
    name: "WinDev Report",
    extensions: &["wde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
