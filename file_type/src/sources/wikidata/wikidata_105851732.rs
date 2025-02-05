use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851732: FileFormat = FileFormat {
    id: 105_851_732,
    source_type: SourceType::Wikidata,
    name: "StepMania Song",
    extensions: &["ssc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
