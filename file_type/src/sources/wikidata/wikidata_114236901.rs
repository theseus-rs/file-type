use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114236901: FileFormat = FileFormat {
    id: 114_236_901,
    puid: "wikidata/114236901",
    name: "Browse Database format",
    extensions: &["bsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
