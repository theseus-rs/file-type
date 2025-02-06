use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48815175: FileFormat = FileFormat {
    id: 48_815_175,
    source_type: SourceType::Wikidata,
    name: "Framework Database, version 2",
    extensions: &["fw", "fw2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
