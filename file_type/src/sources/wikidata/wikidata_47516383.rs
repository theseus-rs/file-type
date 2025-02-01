use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47516383: FileFormat = FileFormat {
    id: 47_516_383,
    puid: "wikidata/47516383",
    name: "Statistical Analysis System Catalog XPT (Windows) v.9.1",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
