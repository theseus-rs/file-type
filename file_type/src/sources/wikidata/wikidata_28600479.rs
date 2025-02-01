use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600479: FileFormat = FileFormat {
    id: 28_600_479,
    puid: "wikidata/28600479",
    name: "DOTX",
    extensions: &["dotx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
