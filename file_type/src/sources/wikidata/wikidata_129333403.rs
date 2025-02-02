use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129333403: FileFormat = FileFormat {
    id: 129_333_403,
    source_type: SourceType::Wikidata,
    name: "Gettext catalog file",
    extensions: &["pot"],
    media_types: &["application/x-gettext", "text/gettext", "text/x-gettext"],
    internal_signatures: &[],
    related_formats: &[],
};
