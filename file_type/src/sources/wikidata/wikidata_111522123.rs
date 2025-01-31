use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111522123: FileFormat = FileFormat {
    id: 111_522_123,
    puid: "wikidata/111522123",
    name: "exFAT (extensible File Allocation Table) disc image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
