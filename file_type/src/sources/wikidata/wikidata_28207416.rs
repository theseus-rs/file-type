use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207416: FileFormat = FileFormat {
    id: 28_207_416,
    puid: "wikidata/28207416",
    name: "VDC BitMap",
    extensions: &["bm", "vbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
