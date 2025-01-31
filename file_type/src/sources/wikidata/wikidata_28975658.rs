use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975658: FileFormat = FileFormat {
    id: 28_975_658,
    puid: "wikidata/28975658",
    name: "SketchUp skp",
    extensions: &["skb", "skp"],
    media_types: &[
        "application/vnd.sketchup.skp",
        "application/vnd.sketchup.skp",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
