use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28345908: FileFormat = FileFormat {
    id: 28_345_908,
    source_type: SourceType::Wikidata,
    name: "Apple Preferred",
    extensions: &["apf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
