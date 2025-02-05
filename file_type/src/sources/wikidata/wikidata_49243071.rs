use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49243071: FileFormat = FileFormat {
    id: 49_243_071,
    source_type: SourceType::Wikidata,
    name: "Java language source code file",
    extensions: &["java"],
    media_types: &["text/x-java"],
    signatures: &[],
    related_formats: &[],
};
