use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418328: FileFormat = FileFormat {
    id: 111_418_328,
    puid: "wikidata/111418328",
    name: "Adobe Bridge Data File",
    extensions: &["abdata"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
