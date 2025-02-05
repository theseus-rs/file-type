use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111182155: FileFormat = FileFormat {
    id: 111_182_155,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Library Item",
    extensions: &["lbi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
