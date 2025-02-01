use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25103897: FileFormat = FileFormat {
    id: 25_103_897,
    puid: "wikidata/25103897",
    name: "Dynamic Text Document",
    extensions: &["dtxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
