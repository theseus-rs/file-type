use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111182292: FileFormat = FileFormat {
    id: 111_182_292,
    puid: "wikidata/111182292",
    name: "Lasso Database-Driven Web Page",
    extensions: &["lasso"],
    media_types: &["text/x-lasso"],
    internal_signatures: &[],
    related_formats: &[],
};
