use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344485: FileFormat = FileFormat {
    id: 28_344_485,
    source_type: SourceType::Wikidata,
    name: "HKI",
    extensions: &["hki"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
