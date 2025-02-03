use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440583: FileFormat = FileFormat {
    id: 111_440_583,
    source_type: SourceType::Wikidata,
    name: "Lua source file",
    extensions: &["lua"],
    media_types: &["application/x-lua", "text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};
