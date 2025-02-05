use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130736665: FileFormat = FileFormat {
    id: 130_736_665,
    source_type: SourceType::Wikidata,
    name: "Savi source code file",
    extensions: &["savi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
