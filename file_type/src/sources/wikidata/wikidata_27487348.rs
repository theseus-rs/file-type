use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27487348: FileFormat = FileFormat {
    id: 27_487_348,
    source_type: SourceType::Wikidata,
    name: "Shapefile spatial index part 2",
    extensions: &["sbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
