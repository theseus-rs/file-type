use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1931585: FileFormat = FileFormat {
    id: 1_931_585,
    source_type: SourceType::Wikidata,
    name: "Microsoft Digital Video Recording",
    extensions: &["dvr-ms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
