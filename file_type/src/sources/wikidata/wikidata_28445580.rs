use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445580: FileFormat = FileFormat {
    id: 28_445_580,
    puid: "wikidata/28445580",
    name: "Application Developer Documentation Data",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
