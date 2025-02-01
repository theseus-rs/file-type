use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1353763: FileFormat = FileFormat {
    id: 1_353_763,
    puid: "wikidata/1353763",
    name: "WMV HD",
    extensions: &["wmv"],
    media_types: &["video/x-ms-wmv"],
    internal_signatures: &[],
    related_formats: &[],
};
