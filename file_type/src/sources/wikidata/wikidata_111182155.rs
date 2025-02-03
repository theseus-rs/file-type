use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111182155: FileFormat = FileFormat {
    id: 111_182_155,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Library Item",
    extensions: &["lbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
