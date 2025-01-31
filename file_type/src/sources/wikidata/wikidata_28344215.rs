use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344215: FileFormat = FileFormat {
    id: 28_344_215,
    puid: "wikidata/28344215",
    name: "Application Developer Documentation Index",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
