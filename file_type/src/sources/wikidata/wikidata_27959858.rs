use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959858: FileFormat = FileFormat {
    id: 27_959_858,
    source_type: SourceType::Wikidata,
    name: "Make-A-Melody song file",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
