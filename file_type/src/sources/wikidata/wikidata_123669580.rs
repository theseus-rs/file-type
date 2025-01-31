use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123669580: FileFormat = FileFormat {
    id: 123_669_580,
    puid: "wikidata/123669580",
    name: "CorelDraw Drawing X7",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
