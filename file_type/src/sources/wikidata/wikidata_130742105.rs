use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130742105: FileFormat = FileFormat {
    id: 130_742_105,
    puid: "wikidata/130742105",
    name: "Scaml markup file",
    extensions: &["scaml"],
    media_types: &["text/x-scaml"],
    internal_signatures: &[],
    related_formats: &[],
};
