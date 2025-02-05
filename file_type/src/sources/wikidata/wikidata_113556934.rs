use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113556934: FileFormat = FileFormat {
    id: 113_556_934,
    source_type: SourceType::Wikidata,
    name: "BlindRead ImageCreator Image",
    extensions: &["iso"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
