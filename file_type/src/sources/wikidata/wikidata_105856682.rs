use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856682: FileFormat = FileFormat {
    id: 105_856_682,
    puid: "wikidata/105856682",
    name: "Uniform Office Format (generic)",
    extensions: &["uof"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
