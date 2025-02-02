use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207350: FileFormat = FileFormat {
    id: 28_207_350,
    source_type: SourceType::Wikidata,
    name: "Video Display Adapter",
    extensions: &["vda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
