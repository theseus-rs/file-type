use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123360595: FileFormat = FileFormat {
    id: 123_360_595,
    puid: "wikidata/123360595",
    name: "PHP 5 Web Page",
    extensions: &["php5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
