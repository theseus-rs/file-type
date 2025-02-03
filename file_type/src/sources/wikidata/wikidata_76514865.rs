use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76514865: FileFormat = FileFormat {
    id: 76_514_865,
    source_type: SourceType::Wikidata,
    name: "WinDev Report",
    extensions: &["wde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
