use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333329: FileFormat = FileFormat {
    id: 111_333_329,
    source_type: SourceType::Wikidata,
    name: "PSION A-law file",
    extensions: &["psion"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
