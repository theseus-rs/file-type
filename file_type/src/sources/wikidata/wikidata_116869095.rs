use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116869095: FileFormat = FileFormat {
    id: 116_869_095,
    puid: "wikidata/116869095",
    name: "Summitsoft Letterhead",
    extensions: &["lth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
