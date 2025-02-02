use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109585918: FileFormat = FileFormat {
    id: 109_585_918,
    source_type: SourceType::Wikidata,
    name: "Painter framestack file format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
