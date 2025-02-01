use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131145237: FileFormat = FileFormat {
    id: 131_145_237,
    puid: "wikidata/131145237",
    name: "SourcePawn source code file",
    extensions: &["sp"],
    media_types: &["text/x-sourcepawn"],
    internal_signatures: &[],
    related_formats: &[],
};
