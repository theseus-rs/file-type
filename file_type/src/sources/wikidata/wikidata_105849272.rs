use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849272: FileFormat = FileFormat {
    id: 105_849_272,
    source_type: SourceType::Wikidata,
    name: "GNU Bison grammar",
    extensions: &["yy"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
