use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120635955: FileFormat = FileFormat {
    id: 120_635_955,
    source_type: SourceType::Wikidata,
    name: "Microsoft Data Access Page",
    extensions: &["htm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
