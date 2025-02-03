use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61813289: FileFormat = FileFormat {
    id: 61_813_289,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 4",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
