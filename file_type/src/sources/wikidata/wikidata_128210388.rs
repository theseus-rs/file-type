use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128210388: FileFormat = FileFormat {
    id: 128_210_388,
    source_type: SourceType::Wikidata,
    name: "Xcode config",
    extensions: &["xcconfig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
