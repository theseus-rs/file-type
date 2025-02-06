use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111167647: FileFormat = FileFormat {
    id: 111_167_647,
    source_type: SourceType::Wikidata,
    name: "ISIS/Sketch file",
    extensions: &["skc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
