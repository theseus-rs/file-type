use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651319: FileFormat = FileFormat {
    id: 29_651_319,
    puid: "wikidata/29651319",
    name: "PixieScript",
    extensions: &["pxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
