use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7295259: FileFormat = FileFormat {
    id: 7_295_259,
    source_type: SourceType::Wikidata,
    name: "Raster Document Object",
    extensions: &["rdo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
