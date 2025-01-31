use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849694: FileFormat = FileFormat {
    id: 105_849_694,
    puid: "wikidata/105849694",
    name: "Celestia script",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
