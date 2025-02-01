use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3928271: FileFormat = FileFormat {
    id: 3_928_271,
    puid: "wikidata/3928271",
    name: "RGBE image format",
    extensions: &["hdr", "pic", "rad", "rgbe", "xyze"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
