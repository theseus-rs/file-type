use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116701684: FileFormat = FileFormat {
    id: 116_701_684,
    source_type: SourceType::Wikidata,
    name: "Mascot Generic Format",
    extensions: &["mgf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
