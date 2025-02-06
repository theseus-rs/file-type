use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440583: FileFormat = FileFormat {
    id: 111_440_583,
    source_type: SourceType::Wikidata,
    name: "Lua source file",
    extensions: &["lua"],
    media_types: &["application/x-lua", "text/x-lua"],
    signatures: &[],
    related_formats: &[],
};
