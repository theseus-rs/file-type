use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50825837: FileFormat = FileFormat {
    id: 50_825_837,
    source_type: SourceType::Wikidata,
    name: "AVCHD Movie Object File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
