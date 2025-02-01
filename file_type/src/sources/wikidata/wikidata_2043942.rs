use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2043942: FileFormat = FileFormat {
    id: 2_043_942,
    puid: "wikidata/2043942",
    name: "Portable Document Format for Engineering",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
