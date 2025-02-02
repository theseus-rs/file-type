use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120867895: FileFormat = FileFormat {
    id: 120_867_895,
    source_type: SourceType::Wikidata,
    name: "Cumulus Record Exchange File",
    extensions: &["cre"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
