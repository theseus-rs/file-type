use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111594729: FileFormat = FileFormat {
    id: 111_594_729,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version 1.5",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
