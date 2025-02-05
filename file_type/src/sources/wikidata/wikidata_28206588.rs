use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206588: FileFormat = FileFormat {
    id: 28_206_588,
    source_type: SourceType::Wikidata,
    name: "Microsoft Image Composer file",
    extensions: &["mic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
