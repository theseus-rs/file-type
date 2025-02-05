use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111182292: FileFormat = FileFormat {
    id: 111_182_292,
    source_type: SourceType::Wikidata,
    name: "Lasso Database-Driven Web Page",
    extensions: &["lasso"],
    media_types: &["text/x-lasso"],
    signatures: &[],
    related_formats: &[],
};
