use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_367: FileFormat = FileFormat {
    id: 1_114,
    puid: "fmt/367",
    name: "ESRI World File Format",
    extensions: &[
        "tfw", "jgw", "pgw", "bpw", "tifw", "blw", "bilw", "jpgw", "rasterw", "btw", "sdw",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
