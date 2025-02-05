use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_959950: FileFormat = FileFormat {
    id: 959_950,
    source_type: SourceType::Wikidata,
    name: "eXtensible Business Reporting Language",
    extensions: &["xbrl", "xml"],
    media_types: &["application/xml"],
    signatures: &[],
    related_formats: &[],
};
