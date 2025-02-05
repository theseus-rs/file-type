use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919071: FileFormat = FileFormat {
    id: 28_919_071,
    source_type: SourceType::Wikidata,
    name: "After Effects project, binary variant",
    extensions: &["aep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
