use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861238: FileFormat = FileFormat {
    id: 105_861_238,
    puid: "wikidata/105861238",
    name: "Camtasia Zipped Library",
    extensions: &["libzip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
