use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206528: FileFormat = FileFormat {
    id: 28_206_528,
    source_type: SourceType::Wikidata,
    name: "MacPaint",
    extensions: &["mac", "pic", "pntg"],
    media_types: &["image/x-macpaint"],
    signatures: &[],
    related_formats: &[],
};
