use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919075: FileFormat = FileFormat {
    id: 28_919_075,
    source_type: SourceType::Wikidata,
    name: "After Effects project template",
    extensions: &["aet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
