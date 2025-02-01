use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49988096: FileFormat = FileFormat {
    id: 49_988_096,
    puid: "wikidata/49988096",
    name: "Apple iBooks format",
    extensions: &["ibooks"],
    media_types: &["application/x-ibooks+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
