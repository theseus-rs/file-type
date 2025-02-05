use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487130: FileFormat = FileFormat {
    id: 27_487_130,
    source_type: SourceType::Wikidata,
    name: "Shapefile dBASE table",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
