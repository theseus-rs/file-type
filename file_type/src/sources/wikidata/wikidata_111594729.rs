use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111594729: FileFormat = FileFormat {
    id: 111_594_729,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version 1.5",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
