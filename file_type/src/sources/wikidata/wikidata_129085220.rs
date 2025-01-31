use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129085220: FileFormat = FileFormat {
    id: 129_085_220,
    puid: "wikidata/129085220",
    name: "elpi file format",
    extensions: &["elpi"],
    media_types: &["text/x-elpi"],
    internal_signatures: &[],
    related_formats: &[],
};
