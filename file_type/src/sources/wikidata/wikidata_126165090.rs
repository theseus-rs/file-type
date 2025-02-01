use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126165090: FileFormat = FileFormat {
    id: 126_165_090,
    puid: "wikidata/126165090",
    name: "Husqvarna-Viking Designer 1 Stitch File",
    extensions: &["mhv", "phv", "shv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
