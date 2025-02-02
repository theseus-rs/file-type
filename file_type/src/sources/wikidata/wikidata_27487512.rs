use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27487512: FileFormat = FileFormat {
    id: 27_487_512,
    source_type: SourceType::Wikidata,
    name: "Shapefile header index",
    extensions: &["aih"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
