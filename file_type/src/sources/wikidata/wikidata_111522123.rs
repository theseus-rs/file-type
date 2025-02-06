use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111522123: FileFormat = FileFormat {
    id: 111_522_123,
    source_type: SourceType::Wikidata,
    name: "exFAT (extensible File Allocation Table) disc image",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
