use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_479833: FileFormat = FileFormat {
    id: 479_833,
    source_type: SourceType::Wikidata,
    name: "batch file",
    extensions: &["bat", "btm", "cmd", "vbs"],
    media_types: &["application/x-bat"],
    internal_signatures: &[],
    related_formats: &[],
};
