use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111596697: FileFormat = FileFormat {
    id: 111_596_697,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version 2",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
