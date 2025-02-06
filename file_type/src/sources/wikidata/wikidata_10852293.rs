use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10852293: FileFormat = FileFormat {
    id: 10_852_293,
    source_type: SourceType::Wikidata,
    name: "RPT",
    extensions: &["rpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
