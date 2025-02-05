use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966991: FileFormat = FileFormat {
    id: 27_966_991,
    source_type: SourceType::Wikidata,
    name: "Art & Magic",
    extensions: &["aam"],
    media_types: &["audio/x-mod"],
    signatures: &[],
    related_formats: &[],
};
