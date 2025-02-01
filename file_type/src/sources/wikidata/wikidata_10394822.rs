use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10394822: FileFormat = FileFormat {
    id: 10_394_822,
    puid: "wikidata/10394822",
    name: "ZIP archive file format, version 6.3.2",
    extensions: &["zip", "zipx"],
    media_types: &["application/zip", "application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
