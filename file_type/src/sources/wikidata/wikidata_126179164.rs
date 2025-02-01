use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126179164: FileFormat = FileFormat {
    id: 126_179_164,
    puid: "wikidata/126179164",
    name: "Macintosh PICT Image 2.0",
    extensions: &["pct", "pic", "pict"],
    media_types: &["image/x-pict", "image/x-pict", "image/x-pict"],
    internal_signatures: &[],
    related_formats: &[],
};
