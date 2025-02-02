use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111182292: FileFormat = FileFormat {
    id: 111_182_292,
    source_type: SourceType::Wikidata,
    name: "Lasso Database-Driven Web Page",
    extensions: &["lasso"],
    media_types: &["text/x-lasso"],
    internal_signatures: &[],
    related_formats: &[],
};
