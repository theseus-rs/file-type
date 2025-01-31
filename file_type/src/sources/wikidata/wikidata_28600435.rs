use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600435: FileFormat = FileFormat {
    id: 28_600_435,
    puid: "wikidata/28600435",
    name: "Consolidated.db",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
