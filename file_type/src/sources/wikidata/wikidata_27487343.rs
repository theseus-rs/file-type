use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487343: FileFormat = FileFormat {
    id: 27_487_343,
    source_type: SourceType::Wikidata,
    name: "Shapefile spatial index part 1",
    extensions: &["sbn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
