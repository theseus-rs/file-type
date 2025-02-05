use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205503: FileFormat = FileFormat {
    id: 28_205_503,
    source_type: SourceType::Wikidata,
    name: "GEM resource file",
    extensions: &["rsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
