use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99851769: FileFormat = FileFormat {
    id: 99_851_769,
    source_type: SourceType::Wikidata,
    name: "ESRI Published Map Format",
    extensions: &["pmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
