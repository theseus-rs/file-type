use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114133839: FileFormat = FileFormat {
    id: 114_133_839,
    source_type: SourceType::Wikidata,
    name: "MacroModels file format",
    extensions: &["mcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
