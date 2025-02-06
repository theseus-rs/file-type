use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487109: FileFormat = FileFormat {
    id: 27_487_109,
    source_type: SourceType::Wikidata,
    name: "Shapefile index file",
    extensions: &["shx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
