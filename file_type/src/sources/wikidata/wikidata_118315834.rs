use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118315834: FileFormat = FileFormat {
    id: 118_315_834,
    source_type: SourceType::Wikidata,
    name: "FotoSlate 4.0 Project",
    extensions: &["plp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
