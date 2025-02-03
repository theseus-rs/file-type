use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50825843: FileFormat = FileFormat {
    id: 50_825_843,
    source_type: SourceType::Wikidata,
    name: "AVCHD Index File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
