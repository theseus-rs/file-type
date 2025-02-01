use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51917759: FileFormat = FileFormat {
    id: 51_917_759,
    puid: "wikidata/51917759",
    name: "SDSC Image Tool X Window Dump Format",
    extensions: &["xwd"],
    media_types: &["image/xwd"],
    internal_signatures: &[],
    related_formats: &[],
};
