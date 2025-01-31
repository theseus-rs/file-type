use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206528: FileFormat = FileFormat {
    id: 28_206_528,
    puid: "wikidata/28206528",
    name: "MacPaint",
    extensions: &["mac", "pic", "pntg"],
    media_types: &["image/x-macpaint", "image/x-macpaint", "image/x-macpaint"],
    internal_signatures: &[],
    related_formats: &[],
};
