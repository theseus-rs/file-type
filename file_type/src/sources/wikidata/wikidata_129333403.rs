use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129333403: FileFormat = FileFormat {
    id: 129_333_403,
    source_type: SourceType::Wikidata,
    name: "Gettext catalog file",
    extensions: &["pot"],
    media_types: &["application/x-gettext", "text/gettext", "text/x-gettext"],
    signatures: &[],
    related_formats: &[],
};
