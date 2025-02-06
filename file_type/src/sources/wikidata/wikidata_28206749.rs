use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206749: FileFormat = FileFormat {
    id: 28_206_749,
    source_type: SourceType::Wikidata,
    name: "Native Image File Format",
    extensions: &["niff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
