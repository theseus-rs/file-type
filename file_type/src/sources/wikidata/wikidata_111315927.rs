use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111315927: FileFormat = FileFormat {
    id: 111_315_927,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS family instrument",
    extensions: &["ins"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
