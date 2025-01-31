use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856458: FileFormat = FileFormat {
    id: 105_856_458,
    puid: "wikidata/105856458",
    name: "WinISD Driver parameters",
    extensions: &["wdr"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
