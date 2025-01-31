use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115117519: FileFormat = FileFormat {
    id: 115_117_519,
    puid: "wikidata/115117519",
    name: "Help Librarian File",
    extensions: &["dat", "dta", "hlp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
