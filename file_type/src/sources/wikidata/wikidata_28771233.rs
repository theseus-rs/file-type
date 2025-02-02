use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771233: FileFormat = FileFormat {
    id: 28_771_233,
    source_type: SourceType::Wikidata,
    name: "MINC",
    extensions: &["mnc"],
    media_types: &["application/x-minc"],
    internal_signatures: &[],
    related_formats: &[],
};
