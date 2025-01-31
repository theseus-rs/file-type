use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120750742: FileFormat = FileFormat {
    id: 120_750_742,
    puid: "wikidata/120750742",
    name: "OpenRP",
    extensions: &["rp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
