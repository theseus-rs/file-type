use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121914796: FileFormat = FileFormat {
    id: 121_914_796,
    source_type: SourceType::Wikidata,
    name: "Microsoft Agent File",
    extensions: &["acs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
