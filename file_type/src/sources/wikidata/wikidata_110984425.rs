use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110984425: FileFormat = FileFormat {
    id: 110_984_425,
    source_type: SourceType::Wikidata,
    name: "Video ToolBox 2 Project file",
    extensions: &["vtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
