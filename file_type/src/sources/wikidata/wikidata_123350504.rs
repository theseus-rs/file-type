use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123350504: FileFormat = FileFormat {
    id: 123_350_504,
    puid: "wikidata/123350504",
    name: "RootsMagic Chart file",
    extensions: &["rmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
