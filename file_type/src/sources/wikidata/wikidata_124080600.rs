use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124080600: FileFormat = FileFormat {
    id: 124_080_600,
    source_type: SourceType::Wikidata,
    name: "CSL-JSON",
    extensions: &["json"],
    media_types: &["application/vnd.citationstyles.csl+json"],
    internal_signatures: &[],
    related_formats: &[],
};
