use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122074310: FileFormat = FileFormat {
    id: 122_074_310,
    source_type: SourceType::Wikidata,
    name: "SmartScore File",
    extensions: &["fin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
