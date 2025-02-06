use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756608: FileFormat = FileFormat {
    id: 28_756_608,
    source_type: SourceType::Wikidata,
    name: "FoxPro script",
    extensions: &["prg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
