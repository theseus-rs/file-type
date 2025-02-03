use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50825837: FileFormat = FileFormat {
    id: 50_825_837,
    source_type: SourceType::Wikidata,
    name: "AVCHD Movie Object File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
