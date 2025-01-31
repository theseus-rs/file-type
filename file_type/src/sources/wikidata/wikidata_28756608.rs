use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756608: FileFormat = FileFormat {
    id: 28_756_608,
    puid: "wikidata/28756608",
    name: "FoxPro script",
    extensions: &["prg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
