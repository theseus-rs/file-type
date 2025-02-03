use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118436204: FileFormat = FileFormat {
    id: 118_436_204,
    source_type: SourceType::Wikidata,
    name: "Esri ArcMap Label file",
    extensions: &["lxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
