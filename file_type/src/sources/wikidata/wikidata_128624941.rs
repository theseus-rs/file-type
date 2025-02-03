use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128624941: FileFormat = FileFormat {
    id: 128_624_941,
    source_type: SourceType::Wikidata,
    name: "AutoIt file",
    extensions: &["au3"],
    media_types: &["text/x-autoit"],
    internal_signatures: &[],
    related_formats: &[],
};
