use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650303: FileFormat = FileFormat {
    id: 29_650_303,
    source_type: SourceType::Wikidata,
    name: "PSRFITS",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
