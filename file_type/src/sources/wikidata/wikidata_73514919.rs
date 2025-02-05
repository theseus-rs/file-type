use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73514919: FileFormat = FileFormat {
    id: 73_514_919,
    source_type: SourceType::Wikidata,
    name: "Package Resource Index",
    extensions: &["pri"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
