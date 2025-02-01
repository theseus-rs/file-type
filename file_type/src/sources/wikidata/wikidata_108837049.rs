use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837049: FileFormat = FileFormat {
    id: 108_837_049,
    puid: "wikidata/108837049",
    name: "Nero CD-ROM (Hybrid) Compilation",
    extensions: &["nrh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
