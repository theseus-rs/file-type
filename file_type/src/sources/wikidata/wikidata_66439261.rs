use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66439261: FileFormat = FileFormat {
    id: 66_439_261,
    puid: "wikidata/66439261",
    name: "WordPerfect Merge Forms file format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
