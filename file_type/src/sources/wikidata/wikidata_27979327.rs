use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979327: FileFormat = FileFormat {
    id: 27_979_327,
    source_type: SourceType::Wikidata,
    name: "Advanced Function Presentation",
    extensions: &["afp"],
    media_types: &["application/x-afp"],
    internal_signatures: &[],
    related_formats: &[],
};
