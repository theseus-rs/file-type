use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113556934: FileFormat = FileFormat {
    id: 113_556_934,
    source_type: SourceType::Wikidata,
    name: "BlindRead ImageCreator Image",
    extensions: &["iso"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
