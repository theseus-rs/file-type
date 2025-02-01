use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652551: FileFormat = FileFormat {
    id: 112_652_551,
    puid: "wikidata/112652551",
    name: "Astound Actor file format",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
