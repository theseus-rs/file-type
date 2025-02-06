use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122074310: FileFormat = FileFormat {
    id: 122_074_310,
    source_type: SourceType::Wikidata,
    name: "SmartScore File",
    extensions: &["fin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
