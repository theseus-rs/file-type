use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28530510: FileFormat = FileFormat {
    id: 28_530_510,
    source_type: SourceType::Wikidata,
    name: "Structure-data file",
    extensions: &["sdf"],
    media_types: &["chemical/x-mdl-sdfile"],
    signatures: &[],
    related_formats: &[],
};
