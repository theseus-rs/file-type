use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6666791: FileFormat = FileFormat {
    id: 6_666_791,
    puid: "wikidata/6666791",
    name: "Log ASCII Standard Format",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
