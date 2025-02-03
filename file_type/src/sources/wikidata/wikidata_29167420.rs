use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167420: FileFormat = FileFormat {
    id: 29_167_420,
    source_type: SourceType::Wikidata,
    name: "Internet Adventure Game Engine source code",
    extensions: &["ic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
