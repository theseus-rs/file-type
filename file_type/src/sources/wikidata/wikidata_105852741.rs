use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852741: FileFormat = FileFormat {
    id: 105_852_741,
    source_type: SourceType::Wikidata,
    name: "Snagit Windows Profile",
    extensions: &["snagprof"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
