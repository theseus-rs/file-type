use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272654: FileFormat = FileFormat {
    id: 111_272_654,
    source_type: SourceType::Wikidata,
    name: "ESPS audio file",
    extensions: &["esps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
