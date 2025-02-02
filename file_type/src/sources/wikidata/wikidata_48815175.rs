use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48815175: FileFormat = FileFormat {
    id: 48_815_175,
    source_type: SourceType::Wikidata,
    name: "Framework Database, version 2",
    extensions: &["fw", "fw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
