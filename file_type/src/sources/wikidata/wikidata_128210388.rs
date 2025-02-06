use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128210388: FileFormat = FileFormat {
    id: 128_210_388,
    source_type: SourceType::Wikidata,
    name: "Xcode config",
    extensions: &["xcconfig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
