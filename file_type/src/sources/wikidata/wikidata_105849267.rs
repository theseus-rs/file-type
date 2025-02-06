use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849267: FileFormat = FileFormat {
    id: 105_849_267,
    source_type: SourceType::Wikidata,
    name: "GNU Bison grammar (with rem)",
    extensions: &["yy"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
