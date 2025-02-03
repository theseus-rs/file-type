use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979156: FileFormat = FileFormat {
    id: 27_979_156,
    source_type: SourceType::Wikidata,
    name: "ASCII art",
    extensions: &["asc", "ascii", "txt"],
    media_types: &["text/vnd.ascii-art"],
    internal_signatures: &[],
    related_formats: &[],
};
