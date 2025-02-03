use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66689214: FileFormat = FileFormat {
    id: 66_689_214,
    source_type: SourceType::Wikidata,
    name: "Access Blank Database Template",
    extensions: &["mdn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
