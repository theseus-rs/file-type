use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849588: FileFormat = FileFormat {
    id: 105_849_588,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher Caption",
    extensions: &["cap"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
