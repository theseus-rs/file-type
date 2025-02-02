use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28018470: FileFormat = FileFormat {
    id: 28_018_470,
    source_type: SourceType::Wikidata,
    name: "MPEG-1 program stream",
    extensions: &["mp1", "mpeg", "mpg"],
    media_types: &["video/MP1S"],
    internal_signatures: &[],
    related_formats: &[],
};
