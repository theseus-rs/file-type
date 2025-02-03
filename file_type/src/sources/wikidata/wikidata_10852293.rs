use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10852293: FileFormat = FileFormat {
    id: 10_852_293,
    source_type: SourceType::Wikidata,
    name: "RPT",
    extensions: &["rpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
