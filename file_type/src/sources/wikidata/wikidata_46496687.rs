use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46496687: FileFormat = FileFormat {
    id: 46_496_687,
    puid: "wikidata/46496687",
    name: "Resource Adapter Archive",
    extensions: &["rar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
