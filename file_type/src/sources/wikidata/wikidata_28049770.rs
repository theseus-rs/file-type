use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049770: FileFormat = FileFormat {
    id: 28_049_770,
    source_type: SourceType::Wikidata,
    name: "DKBTrace scene description",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
