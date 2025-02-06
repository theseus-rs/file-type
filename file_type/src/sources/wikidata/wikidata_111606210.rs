use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111606210: FileFormat = FileFormat {
    id: 111_606_210,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2021",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
