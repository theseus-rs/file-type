use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919160: FileFormat = FileFormat {
    id: 28_919_160,
    source_type: SourceType::Wikidata,
    name: "Standard ACIS Binary",
    extensions: &["sab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
