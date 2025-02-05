use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60615282: FileFormat = FileFormat {
    id: 60_615_282,
    source_type: SourceType::Wikidata,
    name: "Write for Windows Document, version 3",
    extensions: &["wri"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
