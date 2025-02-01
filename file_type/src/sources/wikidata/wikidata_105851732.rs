use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851732: FileFormat = FileFormat {
    id: 105_851_732,
    puid: "wikidata/105851732",
    name: "StepMania Song",
    extensions: &["ssc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
