use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131519225: FileFormat = FileFormat {
    id: 131_519_225,
    source_type: SourceType::Wikidata,
    name: "Stimulate Signal Data",
    extensions: &["sdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
