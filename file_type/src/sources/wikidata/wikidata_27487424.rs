use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487424: FileFormat = FileFormat {
    id: 27_487_424,
    source_type: SourceType::Wikidata,
    name: "Shapefile projections definitions file",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
