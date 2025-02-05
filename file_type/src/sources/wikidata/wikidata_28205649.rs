use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205649: FileFormat = FileFormat {
    id: 28_205_649,
    source_type: SourceType::Wikidata,
    name: "AAI",
    extensions: &["aai"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
