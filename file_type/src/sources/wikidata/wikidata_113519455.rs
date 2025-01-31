use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113519455: FileFormat = FileFormat {
    id: 113_519_455,
    puid: "wikidata/113519455",
    name: "PageMaker Mac Document 6.0",
    extensions: &["pm6", "pt6"],
    media_types: &["application/vnd.pagemaker", "application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
