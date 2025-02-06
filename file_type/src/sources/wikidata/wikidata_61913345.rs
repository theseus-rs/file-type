use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61913345: FileFormat = FileFormat {
    id: 61_913_345,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project Export File, version 3",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    signatures: &[],
    related_formats: &[],
};
