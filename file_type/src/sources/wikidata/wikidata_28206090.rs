use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206090: FileFormat = FileFormat {
    id: 28_206_090,
    source_type: SourceType::Wikidata,
    name: "TT Low Resolution",
    extensions: &["PI8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
