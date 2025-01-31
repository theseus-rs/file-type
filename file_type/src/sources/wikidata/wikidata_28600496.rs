use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600496: FileFormat = FileFormat {
    id: 28_600_496,
    puid: "wikidata/28600496",
    name: "diff",
    extensions: &["diff", "patch"],
    media_types: &["text/x-patch", "text/x-patch"],
    internal_signatures: &[],
    related_formats: &[],
};
