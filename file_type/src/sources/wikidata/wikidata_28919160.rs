use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919160: FileFormat = FileFormat {
    id: 28_919_160,
    source_type: SourceType::Wikidata,
    name: "Standard ACIS Binary",
    extensions: &["sab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
