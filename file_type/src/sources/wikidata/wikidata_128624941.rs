use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128624941: FileFormat = FileFormat {
    id: 128_624_941,
    source_type: SourceType::Wikidata,
    name: "AutoIt file",
    extensions: &["au3"],
    media_types: &["text/x-autoit"],
    signatures: &[],
    related_formats: &[],
};
