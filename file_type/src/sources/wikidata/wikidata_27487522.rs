use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487522: FileFormat = FileFormat {
    id: 27_487_522,
    source_type: SourceType::Wikidata,
    name: "Shapefile attribute index part 1",
    extensions: &["ain"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
