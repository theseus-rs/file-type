use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7514956: FileFormat = FileFormat {
    id: 7_514_956,
    source_type: SourceType::Wikidata,
    name: "Silicon Graphics Image",
    extensions: &["bw", "rgb", "rgba", "sgi"],
    media_types: &["image/sgi", "image/x-rgb", "image/x-sgi", "image/x-sgi-rgb"],
    internal_signatures: &[],
    related_formats: &[],
};
