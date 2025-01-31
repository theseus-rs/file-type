use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71859512: FileFormat = FileFormat {
    id: 71_859_512,
    puid: "wikidata/71859512",
    name: "CorelDraw Drawing, version X3",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
