use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771316: FileFormat = FileFormat {
    id: 28_771_316,
    source_type: SourceType::Wikidata,
    name: "Micrografx Draw",
    extensions: &["drw"],
    media_types: &["application/x-mgx-designer"],
    internal_signatures: &[],
    related_formats: &[],
};
