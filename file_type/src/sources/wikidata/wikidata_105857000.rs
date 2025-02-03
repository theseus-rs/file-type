use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857000: FileFormat = FileFormat {
    id: 105_857_000,
    source_type: SourceType::Wikidata,
    name: "Generic Printer Description - Unidrv minidriver",
    extensions: &["gpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
