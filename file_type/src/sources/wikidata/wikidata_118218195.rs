use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118218195: FileFormat = FileFormat {
    id: 118_218_195,
    source_type: SourceType::Wikidata,
    name: "FotoFinish Layout Template",
    extensions: &["fdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
