use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_479833: FileFormat = FileFormat {
    id: 479_833,
    source_type: SourceType::Wikidata,
    name: "batch file",
    extensions: &["bat", "btm", "cmd", "vbs"],
    media_types: &["application/x-bat"],
    signatures: &[],
    related_formats: &[],
};
