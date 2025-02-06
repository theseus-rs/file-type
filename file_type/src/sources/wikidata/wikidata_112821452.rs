use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112821452: FileFormat = FileFormat {
    id: 112_821_452,
    source_type: SourceType::Wikidata,
    name: "Pro/ENGINEER rendering data file",
    extensions: &["slp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
