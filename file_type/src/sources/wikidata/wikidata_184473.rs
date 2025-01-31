use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_184473: FileFormat = FileFormat {
    id: 184_473,
    puid: "wikidata/184473",
    name: "OpenDocument",
    extensions: &["fodt", "odt"],
    media_types: &[
        "application/vnd.oasis.opendocument.text",
        "application/vnd.oasis.opendocument.text",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
