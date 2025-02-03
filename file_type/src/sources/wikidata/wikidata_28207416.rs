use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207416: FileFormat = FileFormat {
    id: 28_207_416,
    source_type: SourceType::Wikidata,
    name: "VDC BitMap",
    extensions: &["bm", "vbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
