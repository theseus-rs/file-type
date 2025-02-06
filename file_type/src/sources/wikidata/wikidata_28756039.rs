use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756039: FileFormat = FileFormat {
    id: 28_756_039,
    source_type: SourceType::Wikidata,
    name: "FLA",
    extensions: &["fla"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
