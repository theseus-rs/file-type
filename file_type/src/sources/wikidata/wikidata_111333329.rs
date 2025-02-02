use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333329: FileFormat = FileFormat {
    id: 111_333_329,
    source_type: SourceType::Wikidata,
    name: "PSION A-law file",
    extensions: &["psion"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
