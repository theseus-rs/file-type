use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131519225: FileFormat = FileFormat {
    id: 131_519_225,
    source_type: SourceType::Wikidata,
    name: "Stimulate Signal Data",
    extensions: &["sdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
