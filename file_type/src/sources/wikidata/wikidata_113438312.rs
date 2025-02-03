use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113438312: FileFormat = FileFormat {
    id: 113_438_312,
    source_type: SourceType::Wikidata,
    name: "EndNote Compressed Library X - X9",
    extensions: &["enlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
