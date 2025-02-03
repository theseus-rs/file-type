use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111596714: FileFormat = FileFormat {
    id: 111_596_714,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS 5.5",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
