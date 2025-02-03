use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66439261: FileFormat = FileFormat {
    id: 66_439_261,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Merge Forms file format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
