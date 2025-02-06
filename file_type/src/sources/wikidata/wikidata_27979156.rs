use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979156: FileFormat = FileFormat {
    id: 27_979_156,
    source_type: SourceType::Wikidata,
    name: "ASCII art",
    extensions: &["asc", "ascii", "txt"],
    media_types: &["text/vnd.ascii-art"],
    signatures: &[],
    related_formats: &[],
};
