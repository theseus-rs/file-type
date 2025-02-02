use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86996065: FileFormat = FileFormat {
    id: 86_996_065,
    source_type: SourceType::Wikidata,
    name: "PaperPort MAX 5-7",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
