use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59913607: FileFormat = FileFormat {
    id: 59_913_607,
    puid: "wikidata/59913607",
    name: "AV1 Image File Format",
    extensions: &["avif"],
    media_types: &["image/avif"],
    internal_signatures: &[],
    related_formats: &[],
};
