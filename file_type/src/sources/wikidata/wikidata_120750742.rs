use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120750742: FileFormat = FileFormat {
    id: 120_750_742,
    source_type: SourceType::Wikidata,
    name: "OpenRP",
    extensions: &["rp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
