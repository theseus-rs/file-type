use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009482: FileFormat = FileFormat {
    id: 28_009_482,
    source_type: SourceType::Wikidata,
    name: "SimCity 2000 Saved City",
    extensions: &["sc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
