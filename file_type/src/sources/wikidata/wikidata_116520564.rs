use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116520564: FileFormat = FileFormat {
    id: 116_520_564,
    puid: "wikidata/116520564",
    name: "PHP 3 Web Page",
    extensions: &["php3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
