use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61766587: FileFormat = FileFormat {
    id: 61_766_587,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
