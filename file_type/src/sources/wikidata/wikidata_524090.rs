use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_524090: FileFormat = FileFormat {
    id: 524_090,
    source_type: SourceType::Wikidata,
    name: "MT9",
    extensions: &["mt9"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
