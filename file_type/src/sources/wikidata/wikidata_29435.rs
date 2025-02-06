use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29435: FileFormat = FileFormat {
    id: 29_435,
    source_type: SourceType::Wikidata,
    name: "Dolby TrueHD",
    extensions: &["thd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
