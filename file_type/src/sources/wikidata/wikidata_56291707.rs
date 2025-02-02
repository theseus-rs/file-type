use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56291707: FileFormat = FileFormat {
    id: 56_291_707,
    source_type: SourceType::Wikidata,
    name: "Common Workflow Language",
    extensions: &["cwl"],
    media_types: &["application/cwl", "application/cwl+json"],
    internal_signatures: &[],
    related_formats: &[],
};
