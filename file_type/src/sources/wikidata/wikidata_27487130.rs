use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27487130: FileFormat = FileFormat {
    id: 27_487_130,
    source_type: SourceType::Wikidata,
    name: "Shapefile dBASE table",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
