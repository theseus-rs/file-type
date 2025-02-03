use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111519850: FileFormat = FileFormat {
    id: 111_519_850,
    source_type: SourceType::Wikidata,
    name: "Stata .do command file",
    extensions: &["do"],
    media_types: &["application/x-stata", "text/stata", "text/x-stata"],
    internal_signatures: &[],
    related_formats: &[],
};
