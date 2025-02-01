use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967090: FileFormat = FileFormat {
    id: 27_967_090,
    puid: "wikidata/27967090",
    name: "Epic Megagames MASI",
    extensions: &["psm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
