use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205995: FileFormat = FileFormat {
    id: 28_205_995,
    puid: "wikidata/28205995",
    name: "Digital Video Interactive Color Map",
    extensions: &["imc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
