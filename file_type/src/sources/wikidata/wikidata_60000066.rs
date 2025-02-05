use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60000066: FileFormat = FileFormat {
    id: 60_000_066,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Owner File",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
