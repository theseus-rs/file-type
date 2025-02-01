use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1760748: FileFormat = FileFormat {
    id: 1_760_748,
    puid: "wikidata/1760748",
    name: "Konqueror website archive",
    extensions: &["war"],
    media_types: &["application/x-webarchive"],
    internal_signatures: &[],
    related_formats: &[],
};
