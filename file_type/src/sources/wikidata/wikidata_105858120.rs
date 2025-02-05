use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858120: FileFormat = FileFormat {
    id: 105_858_120,
    source_type: SourceType::Wikidata,
    name: "KLH10 RAW tape image directory (with rem)",
    extensions: &["tdr"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
