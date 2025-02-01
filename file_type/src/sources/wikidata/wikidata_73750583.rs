use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73750583: FileFormat = FileFormat {
    id: 73_750_583,
    puid: "wikidata/73750583",
    name: "Intuit QuickBooks for Windows",
    extensions: &["qbw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
