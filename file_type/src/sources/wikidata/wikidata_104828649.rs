use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104828649: FileFormat = FileFormat {
    id: 104_828_649,
    puid: "wikidata/104828649",
    name: "Renoise DSP device chain",
    extensions: &["xrnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
