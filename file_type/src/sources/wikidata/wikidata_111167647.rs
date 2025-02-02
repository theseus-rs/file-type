use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111167647: FileFormat = FileFormat {
    id: 111_167_647,
    source_type: SourceType::Wikidata,
    name: "ISIS/Sketch file",
    extensions: &["skc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
