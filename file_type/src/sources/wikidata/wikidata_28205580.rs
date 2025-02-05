use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205580: FileFormat = FileFormat {
    id: 28_205_580,
    source_type: SourceType::Wikidata,
    name: "OS/2 Icon",
    extensions: &["ico"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
