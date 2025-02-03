use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919071: FileFormat = FileFormat {
    id: 28_919_071,
    source_type: SourceType::Wikidata,
    name: "After Effects project, binary variant",
    extensions: &["aep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
