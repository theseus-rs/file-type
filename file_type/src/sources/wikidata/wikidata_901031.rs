use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_901031: FileFormat = FileFormat {
    id: 901_031,
    source_type: SourceType::Wikidata,
    name: "device independent file format",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
