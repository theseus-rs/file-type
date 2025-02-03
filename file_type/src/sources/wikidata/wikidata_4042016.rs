use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4042016: FileFormat = FileFormat {
    id: 4_042_016,
    source_type: SourceType::Wikidata,
    name: "KSS",
    extensions: &["kss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
