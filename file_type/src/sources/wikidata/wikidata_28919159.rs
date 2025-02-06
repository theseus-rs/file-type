use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919159: FileFormat = FileFormat {
    id: 28_919_159,
    source_type: SourceType::Wikidata,
    name: "Standard ACIS Text",
    extensions: &["sat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
