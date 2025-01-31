use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128624941: FileFormat = FileFormat {
    id: 128_624_941,
    puid: "wikidata/128624941",
    name: "AutoIt file",
    extensions: &["au3"],
    media_types: &["text/x-autoit"],
    internal_signatures: &[],
    related_formats: &[],
};
