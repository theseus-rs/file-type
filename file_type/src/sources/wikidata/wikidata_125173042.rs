use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125173042: FileFormat = FileFormat {
    id: 125_173_042,
    source_type: SourceType::Wikidata,
    name: "Tomboy note",
    extensions: &["note"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
