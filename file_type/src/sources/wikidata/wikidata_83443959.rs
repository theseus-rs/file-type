use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83443959: FileFormat = FileFormat {
    id: 83_443_959,
    source_type: SourceType::Wikidata,
    name: "Terse Executable",
    extensions: &["efi", "te"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
