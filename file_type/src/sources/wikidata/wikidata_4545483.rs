use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4545483: FileFormat = FileFormat {
    id: 4_545_483,
    puid: "wikidata/4545483",
    name: "X File Format",
    extensions: &["x"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
