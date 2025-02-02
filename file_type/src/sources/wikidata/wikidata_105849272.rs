use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849272: FileFormat = FileFormat {
    id: 105_849_272,
    source_type: SourceType::Wikidata,
    name: "GNU Bison grammar",
    extensions: &["yy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
