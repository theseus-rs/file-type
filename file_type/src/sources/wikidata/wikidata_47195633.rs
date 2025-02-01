use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47195633: FileFormat = FileFormat {
    id: 47_195_633,
    puid: "wikidata/47195633",
    name: "AppleWorks Drawing file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
