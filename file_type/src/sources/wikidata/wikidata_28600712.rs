use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600712: FileFormat = FileFormat {
    id: 28_600_712,
    puid: "wikidata/28600712",
    name: "DoItAgain",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
