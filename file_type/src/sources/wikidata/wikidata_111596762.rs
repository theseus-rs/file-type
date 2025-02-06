use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111596762: FileFormat = FileFormat {
    id: 111_596_762,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
