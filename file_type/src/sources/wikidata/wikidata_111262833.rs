use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262833: FileFormat = FileFormat {
    id: 111_262_833,
    source_type: SourceType::Wikidata,
    name: "Velvet Studio instrument",
    extensions: &["ais"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
