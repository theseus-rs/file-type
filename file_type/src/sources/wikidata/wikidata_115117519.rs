use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115117519: FileFormat = FileFormat {
    id: 115_117_519,
    source_type: SourceType::Wikidata,
    name: "Help Librarian File",
    extensions: &["dat", "dta", "hlp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
