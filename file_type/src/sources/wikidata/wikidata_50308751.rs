use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50308751: FileFormat = FileFormat {
    id: 50_308_751,
    puid: "wikidata/50308751",
    name: "MIME Email format",
    extensions: &["eml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
