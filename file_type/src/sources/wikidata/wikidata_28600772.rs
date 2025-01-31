use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600772: FileFormat = FileFormat {
    id: 28_600_772,
    puid: "wikidata/28600772",
    name: "EnCase hash map",
    extensions: &["EnMap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
