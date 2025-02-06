use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849588: FileFormat = FileFormat {
    id: 105_849_588,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher Caption",
    extensions: &["cap"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
