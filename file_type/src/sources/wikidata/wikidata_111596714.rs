use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111596714: FileFormat = FileFormat {
    id: 111_596_714,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS 5.5",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
