use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967085: FileFormat = FileFormat {
    id: 27_967_085,
    puid: "wikidata/27967085",
    name: "Jason Page",
    extensions: &["jpn", "jpo", "smp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
