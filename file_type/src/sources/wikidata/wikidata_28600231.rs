use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600231: FileFormat = FileFormat {
    id: 28_600_231,
    source_type: SourceType::Wikidata,
    name: "APW",
    extensions: &["apw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
