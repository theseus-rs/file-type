use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118436204: FileFormat = FileFormat {
    id: 118_436_204,
    source_type: SourceType::Wikidata,
    name: "Esri ArcMap Label file",
    extensions: &["lxp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
