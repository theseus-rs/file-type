use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123377812: FileFormat = FileFormat {
    id: 123_377_812,
    source_type: SourceType::Wikidata,
    name: "Lightwright Library file",
    extensions: &["lwb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
