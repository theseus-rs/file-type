use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60615282: FileFormat = FileFormat {
    id: 60_615_282,
    source_type: SourceType::Wikidata,
    name: "Write for Windows Document, version 3",
    extensions: &["wri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
