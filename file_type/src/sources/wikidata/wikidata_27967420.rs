use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967420: FileFormat = FileFormat {
    id: 27_967_420,
    source_type: SourceType::Wikidata,
    name: "ANSI Music",
    extensions: &["ams", "mus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
