use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50288247: FileFormat = FileFormat {
    id: 50_288_247,
    source_type: SourceType::Wikidata,
    name: "Adobe Air, v2",
    extensions: &["air"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
