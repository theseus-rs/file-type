use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341736: FileFormat = FileFormat {
    id: 111_341_736,
    puid: "wikidata/111341736",
    name: "Sound Designer II flattened file",
    extensions: &["sd2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
