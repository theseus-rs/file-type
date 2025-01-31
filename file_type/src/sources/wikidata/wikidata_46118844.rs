use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46118844: FileFormat = FileFormat {
    id: 46_118_844,
    puid: "wikidata/46118844",
    name: "Lotus Freelance Smartmaster Graphics",
    extensions: &["mas"],
    media_types: &["application/vnd.lotus-freelance"],
    internal_signatures: &[],
    related_formats: &[],
};
