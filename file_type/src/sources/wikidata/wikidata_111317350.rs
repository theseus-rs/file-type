use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317350: FileFormat = FileFormat {
    id: 111_317_350,
    puid: "wikidata/111317350",
    name: "Matlab variable binary file",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
