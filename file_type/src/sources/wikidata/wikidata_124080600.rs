use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124080600: FileFormat = FileFormat {
    id: 124_080_600,
    source_type: SourceType::Wikidata,
    name: "CSL-JSON",
    extensions: &["json"],
    media_types: &["application/vnd.citationstyles.csl+json"],
    signatures: &[],
    related_formats: &[],
};
