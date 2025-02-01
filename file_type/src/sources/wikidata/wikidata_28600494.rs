use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600494: FileFormat = FileFormat {
    id: 28_600_494,
    puid: "wikidata/28600494",
    name: "Dev-Cpp project",
    extensions: &["dev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
