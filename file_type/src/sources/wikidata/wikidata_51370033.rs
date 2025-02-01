use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51370033: FileFormat = FileFormat {
    id: 51_370_033,
    puid: "wikidata/51370033",
    name: "Freelance file format",
    extensions: &["pre"],
    media_types: &["application/vnd.lotus-freelance"],
    internal_signatures: &[],
    related_formats: &[],
};
