use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207270: FileFormat = FileFormat {
    id: 28_207_270,
    source_type: SourceType::Wikidata,
    name: "Secret Photos puzzle",
    extensions: &["xp0"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
