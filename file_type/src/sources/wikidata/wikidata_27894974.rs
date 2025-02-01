use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27894974: FileFormat = FileFormat {
    id: 27_894_974,
    puid: "wikidata/27894974",
    name: "Windows Media Redirector",
    extensions: &["wmx"],
    media_types: &["video/x-ms-wmx"],
    internal_signatures: &[],
    related_formats: &[],
};
