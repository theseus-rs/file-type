use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1114: FileFormat = FileFormat {
    id: 1_114,
    source_type: SourceType::Pronom,
    name: "ESRI World File Format",
    extensions: &[
        "tfw", "jgw", "pgw", "bpw", "tifw", "blw", "bilw", "jpgw", "rasterw", "btw", "sdw",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
