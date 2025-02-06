use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860126: FileFormat = FileFormat {
    id: 105_860_126,
    source_type: SourceType::Wikidata,
    name: "RenderWare 3d model (with rem)",
    extensions: &["rwx"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
