use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205649: FileFormat = FileFormat {
    id: 28_205_649,
    source_type: SourceType::Wikidata,
    name: "AAI",
    extensions: &["aai"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
