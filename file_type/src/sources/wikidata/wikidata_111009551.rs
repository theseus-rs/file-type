use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009551: FileFormat = FileFormat {
    id: 111_009_551,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Craft File format",
    extensions: &["cft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
