use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27995530: FileFormat = FileFormat {
    id: 27_995_530,
    puid: "wikidata/27995530",
    name: "Electronic Arts BIG Archive",
    extensions: &["big", "viv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
