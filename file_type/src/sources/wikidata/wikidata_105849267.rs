use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849267: FileFormat = FileFormat {
    id: 105_849_267,
    source_type: SourceType::Wikidata,
    name: "GNU Bison grammar (with rem)",
    extensions: &["yy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
