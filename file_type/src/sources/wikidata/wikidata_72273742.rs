use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72273742: FileFormat = FileFormat {
    id: 72_273_742,
    source_type: SourceType::Wikidata,
    name: "TPN",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
