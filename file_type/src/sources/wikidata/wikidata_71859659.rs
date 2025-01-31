use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71859659: FileFormat = FileFormat {
    id: 71_859_659,
    puid: "wikidata/71859659",
    name: "CorelDraw Drawing, version X4",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
