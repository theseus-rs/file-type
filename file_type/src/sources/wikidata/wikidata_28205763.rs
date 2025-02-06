use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205763: FileFormat = FileFormat {
    id: 28_205_763,
    source_type: SourceType::Wikidata,
    name: "Binary Information File",
    extensions: &["bif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
