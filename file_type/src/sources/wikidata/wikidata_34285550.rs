use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34285550: FileFormat = FileFormat {
    id: 34_285_550,
    puid: "wikidata/34285550",
    name: "Perl header",
    extensions: &["ph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
