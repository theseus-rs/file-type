use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120867970: FileFormat = FileFormat {
    id: 120_867_970,
    source_type: SourceType::Wikidata,
    name: "Cumulus Query Exchange File",
    extensions: &["cqe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
