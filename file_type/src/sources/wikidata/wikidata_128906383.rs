use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128906383: FileFormat = FileFormat {
    id: 128_906_383,
    source_type: SourceType::Wikidata,
    name: "Dylan session file",
    extensions: &["dylan-console"],
    media_types: &["text/x-dylan-console"],
    signatures: &[],
    related_formats: &[],
};
