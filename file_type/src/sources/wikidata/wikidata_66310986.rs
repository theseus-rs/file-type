use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66310986: FileFormat = FileFormat {
    id: 66_310_986,
    puid: "wikidata/66310986",
    name: "Shopping List file format",
    extensions: &["sl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
