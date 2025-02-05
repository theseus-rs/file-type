use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_9353810: FileFormat = FileFormat {
    id: 9_353_810,
    source_type: SourceType::Wikidata,
    name: "Oracle Database Trace File",
    extensions: &["trc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
