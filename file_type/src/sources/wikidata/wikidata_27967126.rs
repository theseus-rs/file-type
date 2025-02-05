use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967126: FileFormat = FileFormat {
    id: 27_967_126,
    source_type: SourceType::Wikidata,
    name: "CMR",
    extensions: &["cmr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
