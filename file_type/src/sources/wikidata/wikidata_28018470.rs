use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28018470: FileFormat = FileFormat {
    id: 28_018_470,
    source_type: SourceType::Wikidata,
    name: "MPEG-1 program stream",
    extensions: &["mp1", "mpeg", "mpg"],
    media_types: &["video/MP1S"],
    signatures: &[],
    related_formats: &[],
};
