use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85708012: FileFormat = FileFormat {
    id: 85_708_012,
    puid: "wikidata/85708012",
    name: "Calendar Creator Event 3-4",
    extensions: &["ce3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
