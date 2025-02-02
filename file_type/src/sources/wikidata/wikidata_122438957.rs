use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122438957: FileFormat = FileFormat {
    id: 122_438_957,
    source_type: SourceType::Wikidata,
    name: "TurboTax 2009 Tax Return",
    extensions: &["tax2009"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
