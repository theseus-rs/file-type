use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121815925: FileFormat = FileFormat {
    id: 121_815_925,
    source_type: SourceType::Wikidata,
    name: "GST Art File 2",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
