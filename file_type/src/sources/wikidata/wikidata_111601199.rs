use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111601199: FileFormat = FileFormat {
    id: 111_601_199,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2017",
    extensions: &["indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
