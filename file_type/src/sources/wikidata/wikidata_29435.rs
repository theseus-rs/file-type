use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29435: FileFormat = FileFormat {
    id: 29_435,
    source_type: SourceType::Wikidata,
    name: "Dolby TrueHD",
    extensions: &["thd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
