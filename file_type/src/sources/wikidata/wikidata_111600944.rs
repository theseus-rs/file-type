use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111600944: FileFormat = FileFormat {
    id: 111_600_944,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2014",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
