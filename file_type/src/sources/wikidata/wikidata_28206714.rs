use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206714: FileFormat = FileFormat {
    id: 28_206_714,
    puid: "wikidata/28206714",
    name: "Portable Anymap",
    extensions: &["pnm"],
    media_types: &["image/x-portable-anymap"],
    internal_signatures: &[],
    related_formats: &[],
};
