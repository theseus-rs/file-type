use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1566078: FileFormat = FileFormat {
    id: 1_566_078,
    puid: "wikidata/1566078",
    name: "HTML Application",
    extensions: &["hta"],
    media_types: &["application/hta"],
    internal_signatures: &[],
    related_formats: &[],
};
