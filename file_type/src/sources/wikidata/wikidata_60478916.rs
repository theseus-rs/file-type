use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60478916: FileFormat = FileFormat {
    id: 60_478_916,
    source_type: SourceType::Wikidata,
    name: "Qsplat Model",
    extensions: &["qs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
