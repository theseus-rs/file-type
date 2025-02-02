use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114133839: FileFormat = FileFormat {
    id: 114_133_839,
    source_type: SourceType::Wikidata,
    name: "MacroModels file format",
    extensions: &["mcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
