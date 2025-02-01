use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124080600: FileFormat = FileFormat {
    id: 124_080_600,
    puid: "wikidata/124080600",
    name: "CSL-JSON",
    extensions: &["json"],
    media_types: &["application/vnd.citationstyles.csl+json"],
    internal_signatures: &[],
    related_formats: &[],
};
