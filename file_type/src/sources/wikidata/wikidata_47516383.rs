use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47516383: FileFormat = FileFormat {
    id: 47_516_383,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog XPT (Windows) v.9.1",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
