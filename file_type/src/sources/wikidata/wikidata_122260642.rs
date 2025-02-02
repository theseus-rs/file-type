use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122260642: FileFormat = FileFormat {
    id: 122_260_642,
    source_type: SourceType::Wikidata,
    name: "JACOsub",
    extensions: &["jss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
