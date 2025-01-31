use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344013: FileFormat = FileFormat {
    id: 28_344_013,
    puid: "wikidata/28344013",
    name: "BACKUP",
    extensions: &["@@@"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
