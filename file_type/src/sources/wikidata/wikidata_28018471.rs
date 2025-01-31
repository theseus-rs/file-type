use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28018471: FileFormat = FileFormat {
    id: 28_018_471,
    puid: "wikidata/28018471",
    name: "MPEG-2 program stream",
    extensions: &["mod", "mp2", "mpeg", "mpg"],
    media_types: &["video/MP2P", "video/MP2P", "video/MP2P", "video/MP2P"],
    internal_signatures: &[],
    related_formats: &[],
};
