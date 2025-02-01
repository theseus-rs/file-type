use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757880: FileFormat = FileFormat {
    id: 28_757_880,
    puid: "wikidata/28757880",
    name: "git packfile",
    extensions: &["pack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
