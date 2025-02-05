use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_56291707: FileFormat = FileFormat {
    id: 56_291_707,
    source_type: SourceType::Wikidata,
    name: "Common Workflow Language",
    extensions: &["cwl"],
    media_types: &["application/cwl", "application/cwl+json"],
    signatures: &[],
    related_formats: &[],
};
