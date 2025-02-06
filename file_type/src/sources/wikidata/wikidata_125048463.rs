use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125048463: FileFormat = FileFormat {
    id: 125_048_463,
    source_type: SourceType::Wikidata,
    name: "Yoshimi Scale Settings file",
    extensions: &["xsz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
