use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111600944: FileFormat = FileFormat {
    id: 111_600_944,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2014",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
