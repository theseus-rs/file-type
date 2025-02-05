use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131145237: FileFormat = FileFormat {
    id: 131_145_237,
    source_type: SourceType::Wikidata,
    name: "SourcePawn source code file",
    extensions: &["sp"],
    media_types: &["text/x-sourcepawn"],
    signatures: &[],
    related_formats: &[],
};
