use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47165323: FileFormat = FileFormat {
    id: 47_165_323,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project file format, version 2000-2003",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
