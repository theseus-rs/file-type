use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849587: FileFormat = FileFormat {
    id: 105_849_587,
    source_type: SourceType::Wikidata,
    name: "Cube LUT format (with rem)",
    extensions: &["cube"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
