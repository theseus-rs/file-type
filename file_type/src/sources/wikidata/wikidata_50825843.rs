use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50825843: FileFormat = FileFormat {
    id: 50_825_843,
    source_type: SourceType::Wikidata,
    name: "AVCHD Index File",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
