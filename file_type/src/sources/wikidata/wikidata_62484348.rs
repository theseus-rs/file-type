use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62484348: FileFormat = FileFormat {
    id: 62_484_348,
    source_type: SourceType::Wikidata,
    name: "AccessData Custom Content Image",
    extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
