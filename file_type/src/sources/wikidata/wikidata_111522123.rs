use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111522123: FileFormat = FileFormat {
    id: 111_522_123,
    source_type: SourceType::Wikidata,
    name: "exFAT (extensible File Allocation Table) disc image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
