use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51370033: FileFormat = FileFormat {
    id: 51_370_033,
    source_type: SourceType::Wikidata,
    name: "Freelance file format",
    extensions: &["pre"],
    media_types: &["application/vnd.lotus-freelance"],
    signatures: &[],
    related_formats: &[],
};
