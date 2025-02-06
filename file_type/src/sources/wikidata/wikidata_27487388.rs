use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487388: FileFormat = FileFormat {
    id: 27_487_388,
    source_type: SourceType::Wikidata,
    name: "Shapefile geocoding index",
    extensions: &["ixs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
