use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111601782: FileFormat = FileFormat {
    id: 111_601_782,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2018",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
