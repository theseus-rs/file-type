use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99976195: FileFormat = FileFormat {
    id: 99_976_195,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.0.1",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
