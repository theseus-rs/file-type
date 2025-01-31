use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757999: FileFormat = FileFormat {
    id: 28_757_999,
    puid: "wikidata/28757999",
    name: "Inform",
    extensions: &["i7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
