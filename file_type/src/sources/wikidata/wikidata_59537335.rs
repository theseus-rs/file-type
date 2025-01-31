use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59537335: FileFormat = FileFormat {
    id: 59_537_335,
    puid: "wikidata/59537335",
    name: "Apple iWorks Keynote",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
