use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28018471: FileFormat = FileFormat {
    id: 28_018_471,
    source_type: SourceType::Wikidata,
    name: "MPEG-2 program stream",
    extensions: &["mod", "mp2", "mpeg", "mpg"],
    media_types: &["video/MP2P"],
    signatures: &[],
    related_formats: &[],
};
