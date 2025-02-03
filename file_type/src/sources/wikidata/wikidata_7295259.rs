use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7295259: FileFormat = FileFormat {
    id: 7_295_259,
    source_type: SourceType::Wikidata,
    name: "Raster Document Object",
    extensions: &["rdo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
