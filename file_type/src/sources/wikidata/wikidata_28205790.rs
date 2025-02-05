use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205790: FileFormat = FileFormat {
    id: 28_205_790,
    source_type: SourceType::Wikidata,
    name: "FullPic Picture Format",
    extensions: &["ful"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
