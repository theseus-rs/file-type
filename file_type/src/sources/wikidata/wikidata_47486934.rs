use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47486934: FileFormat = FileFormat {
    id: 47_486_934,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
