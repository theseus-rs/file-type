use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131626493: FileFormat = FileFormat {
    id: 131_626_493,
    source_type: SourceType::Wikidata,
    name: "Tabulat data file format",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
