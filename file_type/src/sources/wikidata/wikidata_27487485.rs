use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487485: FileFormat = FileFormat {
    id: 27_487_485,
    source_type: SourceType::Wikidata,
    name: "Shapefile spatial index of features part 1",
    extensions: &["fbn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
