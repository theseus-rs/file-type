use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129498387: FileFormat = FileFormat {
    id: 129_498_387,
    puid: "wikidata/129498387",
    name: "Haml file format",
    extensions: &["haml"],
    media_types: &["text/x-haml"],
    internal_signatures: &[],
    related_formats: &[],
};
