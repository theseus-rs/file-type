use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79242927: FileFormat = FileFormat {
    id: 79_242_927,
    puid: "wikidata/79242927",
    name: "Adobe After Effects Graphics",
    extensions: &["aegraphic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
