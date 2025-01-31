use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129177480: FileFormat = FileFormat {
    id: 129_177_480,
    puid: "wikidata/129177480",
    name: "Fennel source code file",
    extensions: &["fnl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
