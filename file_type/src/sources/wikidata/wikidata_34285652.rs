use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34285652: FileFormat = FileFormat {
    id: 34_285_652,
    puid: "wikidata/34285652",
    name: "Perl Common Gateway Interface script",
    extensions: &["cgi", "fcgi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
