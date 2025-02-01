use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28052835: FileFormat = FileFormat {
    id: 28_052_835,
    puid: "wikidata/28052835",
    name: "Digital Replica Plus",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
