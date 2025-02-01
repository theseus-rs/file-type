use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600238: FileFormat = FileFormat {
    id: 28_600_238,
    puid: "wikidata/28600238",
    name: "ARC",
    extensions: &["arc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
