use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904512: FileFormat = FileFormat {
    id: 29_904_512,
    puid: "wikidata/29904512",
    name: "SAR",
    extensions: &["sar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
