use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49812221: FileFormat = FileFormat {
    id: 49_812_221,
    source_type: SourceType::Wikidata,
    name: "Vectorworks file format, version 2009",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    signatures: &[],
    related_formats: &[],
};
