use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100323933: FileFormat = FileFormat {
    id: 100_323_933,
    source_type: SourceType::Wikidata,
    name: "GST Publisher File 2",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
