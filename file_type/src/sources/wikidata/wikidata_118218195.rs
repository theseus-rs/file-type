use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118218195: FileFormat = FileFormat {
    id: 118_218_195,
    source_type: SourceType::Wikidata,
    name: "FotoFinish Layout Template",
    extensions: &["fdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
