use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167420: FileFormat = FileFormat {
    id: 29_167_420,
    source_type: SourceType::Wikidata,
    name: "Internet Adventure Game Engine source code",
    extensions: &["ic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
