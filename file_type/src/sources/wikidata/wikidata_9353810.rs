use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_9353810: FileFormat = FileFormat {
    id: 9_353_810,
    source_type: SourceType::Wikidata,
    name: "Oracle Database Trace File",
    extensions: &["trc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
