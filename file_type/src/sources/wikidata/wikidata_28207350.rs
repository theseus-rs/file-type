use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207350: FileFormat = FileFormat {
    id: 28_207_350,
    source_type: SourceType::Wikidata,
    name: "Video Display Adapter",
    extensions: &["vda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
