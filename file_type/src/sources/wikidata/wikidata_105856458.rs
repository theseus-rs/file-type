use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856458: FileFormat = FileFormat {
    id: 105_856_458,
    source_type: SourceType::Wikidata,
    name: "WinISD Driver parameters",
    extensions: &["wdr"],
    media_types: &["text/ini"],
    signatures: &[],
    related_formats: &[],
};
