use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967090: FileFormat = FileFormat {
    id: 27_967_090,
    source_type: SourceType::Wikidata,
    name: "Epic Megagames MASI",
    extensions: &["psm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
