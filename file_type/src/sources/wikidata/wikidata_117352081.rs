use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117352081: FileFormat = FileFormat {
    id: 117_352_081,
    source_type: SourceType::Wikidata,
    name: "Capture Library",
    extensions: &["olb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
