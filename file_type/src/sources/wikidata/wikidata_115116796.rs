use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115116796: FileFormat = FileFormat {
    id: 115_116_796,
    puid: "wikidata/115116796",
    name: "Gunpaint Image File",
    extensions: &["gun"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
