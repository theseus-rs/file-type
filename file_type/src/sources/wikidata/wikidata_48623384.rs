use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48623384: FileFormat = FileFormat {
    id: 48_623_384,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project Export File",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    signatures: &[],
    related_formats: &[],
};
