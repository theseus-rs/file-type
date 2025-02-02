use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50288247: FileFormat = FileFormat {
    id: 50_288_247,
    source_type: SourceType::Wikidata,
    name: "Adobe Air, v2",
    extensions: &["air"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
