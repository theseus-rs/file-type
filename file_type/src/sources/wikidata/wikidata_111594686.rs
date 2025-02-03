use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111594686: FileFormat = FileFormat {
    id: 111_594_686,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version 1",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
