use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967083: FileFormat = FileFormat {
    id: 27_967_083,
    puid: "wikidata/27967083",
    name: "Digital Illusions",
    extensions: &["di"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
