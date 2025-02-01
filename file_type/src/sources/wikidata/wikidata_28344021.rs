use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344021: FileFormat = FileFormat {
    id: 28_344_021,
    puid: "wikidata/28344021",
    name: "Imagine Object File",
    extensions: &["iob", "obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
