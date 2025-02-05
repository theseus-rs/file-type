use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487348: FileFormat = FileFormat {
    id: 27_487_348,
    source_type: SourceType::Wikidata,
    name: "Shapefile spatial index part 2",
    extensions: &["sbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
