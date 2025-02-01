use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131626493: FileFormat = FileFormat {
    id: 131_626_493,
    puid: "wikidata/131626493",
    name: "Tabulat data file format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
