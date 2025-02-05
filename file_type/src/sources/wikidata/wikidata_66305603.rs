use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66305603: FileFormat = FileFormat {
    id: 66_305_603,
    source_type: SourceType::Wikidata,
    name: "Data Source Name file format",
    extensions: &["dsn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
