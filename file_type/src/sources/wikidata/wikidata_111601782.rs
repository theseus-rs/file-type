use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111601782: FileFormat = FileFormat {
    id: 111_601_782,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2018",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
