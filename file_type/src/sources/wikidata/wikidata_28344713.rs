use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344713: FileFormat = FileFormat {
    id: 28_344_713,
    puid: "wikidata/28344713",
    name: "Package File",
    extensions: &["pkg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
