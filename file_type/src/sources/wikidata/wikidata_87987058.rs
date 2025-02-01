use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87987058: FileFormat = FileFormat {
    id: 87_987_058,
    puid: "wikidata/87987058",
    name: "CorelCHART Document 5",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
