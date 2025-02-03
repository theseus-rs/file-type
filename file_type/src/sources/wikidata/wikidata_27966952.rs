use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966952: FileFormat = FileFormat {
    id: 27_966_952,
    source_type: SourceType::Wikidata,
    name: "SSF",
    extensions: &["minissf", "ssf", "ssflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
