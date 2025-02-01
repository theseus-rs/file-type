use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344449: FileFormat = FileFormat {
    id: 28_344_449,
    puid: "wikidata/28344449",
    name: "SNSF",
    extensions: &["minisnsf", "snsf", "snsflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
