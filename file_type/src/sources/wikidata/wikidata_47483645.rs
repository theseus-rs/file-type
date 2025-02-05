use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47483645: FileFormat = FileFormat {
    id: 47_483_645,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Data (Windows)",
    extensions: &["sas7bdat", "sd7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
