use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66439261: FileFormat = FileFormat {
    id: 66_439_261,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Merge Forms file format",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
