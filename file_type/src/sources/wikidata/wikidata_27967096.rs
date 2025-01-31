use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967096: FileFormat = FileFormat {
    id: 27_967_096,
    puid: "wikidata/27967096",
    name: "Ken's Adlib Music",
    extensions: &["ksm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
