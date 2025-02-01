use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122673484: FileFormat = FileFormat {
    id: 122_673_484,
    puid: "wikidata/122673484",
    name: "Outline 4D Document",
    extensions: &["syv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
