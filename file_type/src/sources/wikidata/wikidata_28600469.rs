use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600469: FileFormat = FileFormat {
    id: 28_600_469,
    puid: "wikidata/28600469",
    name: "Distinguished Encoding Rules",
    extensions: &["der"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
