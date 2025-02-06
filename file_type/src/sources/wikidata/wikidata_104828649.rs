use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104828649: FileFormat = FileFormat {
    id: 104_828_649,
    source_type: SourceType::Wikidata,
    name: "Renoise DSP device chain",
    extensions: &["xrnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
