use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27923693: FileFormat = FileFormat {
    id: 27_923_693,
    source_type: SourceType::Wikidata,
    name: "DTED Cells Directory File",
    extensions: &["dir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
