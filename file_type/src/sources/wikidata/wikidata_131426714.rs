use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131426714: FileFormat = FileFormat {
    id: 131_426_714,
    source_type: SourceType::Wikidata,
    name: "X++ source code file format",
    extensions: &["xpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
