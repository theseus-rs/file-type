use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131626493: FileFormat = FileFormat {
    id: 131_626_493,
    source_type: SourceType::Wikidata,
    name: "Tabulat data file format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
