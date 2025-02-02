use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205790: FileFormat = FileFormat {
    id: 28_205_790,
    source_type: SourceType::Wikidata,
    name: "FullPic Picture Format",
    extensions: &["ful"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
