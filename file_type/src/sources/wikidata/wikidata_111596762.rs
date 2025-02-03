use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111596762: FileFormat = FileFormat {
    id: 111_596_762,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
