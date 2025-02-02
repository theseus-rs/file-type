use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_81526528: FileFormat = FileFormat {
    id: 81_526_528,
    source_type: SourceType::Wikidata,
    name: "MicroStation Resource data",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
