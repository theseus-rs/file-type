use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34747804: FileFormat = FileFormat {
    id: 34_747_804,
    source_type: SourceType::Wikidata,
    name: "Supaplex Level format",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
