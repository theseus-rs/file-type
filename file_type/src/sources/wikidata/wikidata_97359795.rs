use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97359795: FileFormat = FileFormat {
    id: 97_359_795,
    puid: "wikidata/97359795",
    name: "AnIML",
    extensions: &["animl"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
