use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849830: FileFormat = FileFormat {
    id: 105_849_830,
    puid: "wikidata/105849830",
    name: "Camtasia Studio Screen Recording",
    extensions: &["camrec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
