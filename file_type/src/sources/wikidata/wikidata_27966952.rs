use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966952: FileFormat = FileFormat {
    id: 27_966_952,
    source_type: SourceType::Wikidata,
    name: "SSF",
    extensions: &["minissf", "ssf", "ssflib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
