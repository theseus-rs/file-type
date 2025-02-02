use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206528: FileFormat = FileFormat {
    id: 28_206_528,
    source_type: SourceType::Wikidata,
    name: "MacPaint",
    extensions: &["mac", "pic", "pntg"],
    media_types: &["image/x-macpaint"],
    internal_signatures: &[],
    related_formats: &[],
};
