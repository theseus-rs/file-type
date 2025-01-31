use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48223393: FileFormat = FileFormat {
    id: 48_223_393,
    puid: "wikidata/48223393",
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
