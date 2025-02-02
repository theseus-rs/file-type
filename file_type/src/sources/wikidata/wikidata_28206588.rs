use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206588: FileFormat = FileFormat {
    id: 28_206_588,
    source_type: SourceType::Wikidata,
    name: "Microsoft Image Composer file",
    extensions: &["mic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
