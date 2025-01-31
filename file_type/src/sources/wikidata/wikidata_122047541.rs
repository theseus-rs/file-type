use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122047541: FileFormat = FileFormat {
    id: 122_047_541,
    puid: "wikidata/122047541",
    name: "cc:Mail Archive Format",
    extensions: &["cca"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
