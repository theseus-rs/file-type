use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600250: FileFormat = FileFormat {
    id: 28_600_250,
    source_type: SourceType::Wikidata,
    name: "Oracle database backup format",
    extensions: &["arc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
