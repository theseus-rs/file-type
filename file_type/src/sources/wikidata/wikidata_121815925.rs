use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121815925: FileFormat = FileFormat {
    id: 121_815_925,
    source_type: SourceType::Wikidata,
    name: "GST Art File 2",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
