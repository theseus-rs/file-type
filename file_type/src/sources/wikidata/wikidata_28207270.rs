use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207270: FileFormat = FileFormat {
    id: 28_207_270,
    source_type: SourceType::Wikidata,
    name: "Secret Photos puzzle",
    extensions: &["xp0"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
