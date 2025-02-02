use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110984425: FileFormat = FileFormat {
    id: 110_984_425,
    source_type: SourceType::Wikidata,
    name: "Video ToolBox 2 Project file",
    extensions: &["vtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
