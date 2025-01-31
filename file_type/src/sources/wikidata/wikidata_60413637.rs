use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60413637: FileFormat = FileFormat {
    id: 60_413_637,
    puid: "wikidata/60413637",
    name: "INTERLIS Model File, version 2.3",
    extensions: &["ili"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
