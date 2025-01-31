use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128780753: FileFormat = FileFormat {
    id: 128_780_753,
    puid: "wikidata/128780753",
    name: "crmsh configuration file",
    extensions: &["crmsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
