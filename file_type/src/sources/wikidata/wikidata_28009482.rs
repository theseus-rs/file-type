use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009482: FileFormat = FileFormat {
    id: 28_009_482,
    source_type: SourceType::Wikidata,
    name: "SimCity 2000 Saved City",
    extensions: &["sc2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
