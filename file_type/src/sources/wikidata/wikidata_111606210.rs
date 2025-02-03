use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111606210: FileFormat = FileFormat {
    id: 111_606_210,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2021",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
