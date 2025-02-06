use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58237034: FileFormat = FileFormat {
    id: 58_237_034,
    source_type: SourceType::Wikidata,
    name: "Adobe Multiple Master Metrics font file",
    extensions: &["mmm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
