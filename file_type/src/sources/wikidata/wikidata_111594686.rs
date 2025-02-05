use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111594686: FileFormat = FileFormat {
    id: 111_594_686,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version 1",
    extensions: &["ind", "indd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
