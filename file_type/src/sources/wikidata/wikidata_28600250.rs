use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600250: FileFormat = FileFormat {
    id: 28_600_250,
    source_type: SourceType::Wikidata,
    name: "Oracle database backup format",
    extensions: &["arc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
