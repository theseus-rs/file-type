use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119443865: FileFormat = FileFormat {
    id: 119_443_865,
    puid: "wikidata/119443865",
    name: "AutoRoute GB File",
    extensions: &["axg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
