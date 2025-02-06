use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117352081: FileFormat = FileFormat {
    id: 117_352_081,
    source_type: SourceType::Wikidata,
    name: "Capture Library",
    extensions: &["olb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
