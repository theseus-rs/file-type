use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116701684: FileFormat = FileFormat {
    id: 116_701_684,
    source_type: SourceType::Wikidata,
    name: "Mascot Generic Format",
    extensions: &["mgf"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
