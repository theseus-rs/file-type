use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29642901: FileFormat = FileFormat {
    id: 29_642_901,
    source_type: SourceType::Wikidata,
    name: "C header file",
    extensions: &["h", "hpp", "hxx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
