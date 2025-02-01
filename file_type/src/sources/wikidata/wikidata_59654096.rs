use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59654096: FileFormat = FileFormat {
    id: 59_654_096,
    puid: "wikidata/59654096",
    name: "Adobe Content Server Message File",
    extensions: &["acsm"],
    media_types: &["application/vnd.adobe.adept+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
