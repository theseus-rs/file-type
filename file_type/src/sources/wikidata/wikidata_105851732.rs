use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851732: FileFormat = FileFormat {
    id: 105_851_732,
    source_type: SourceType::Wikidata,
    name: "StepMania Song",
    extensions: &["ssc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
