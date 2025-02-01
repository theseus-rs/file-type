use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112669255: FileFormat = FileFormat {
    id: 112_669_255,
    puid: "wikidata/112669255",
    name: "JSR-184 format",
    extensions: &["m3g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
