use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111315927: FileFormat = FileFormat {
    id: 111_315_927,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS family instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
