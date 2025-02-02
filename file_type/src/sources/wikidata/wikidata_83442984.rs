use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83442984: FileFormat = FileFormat {
    id: 83_442_984,
    source_type: SourceType::Wikidata,
    name: "Ducati Data Analyzer",
    extensions: &["dda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
