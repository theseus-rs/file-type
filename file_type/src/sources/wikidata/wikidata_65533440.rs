use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65533440: FileFormat = FileFormat {
    id: 65_533_440,
    source_type: SourceType::Wikidata,
    name: "BigOven Recipe Box File",
    extensions: &["crb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
