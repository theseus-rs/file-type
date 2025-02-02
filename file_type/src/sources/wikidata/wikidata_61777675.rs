use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61777675: FileFormat = FileFormat {
    id: 61_777_675,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 3a",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
