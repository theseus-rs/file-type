use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979395: FileFormat = FileFormat {
    id: 27_979_395,
    source_type: SourceType::Wikidata,
    name: "GIFV",
    extensions: &["gifv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
