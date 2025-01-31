use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837072: FileFormat = FileFormat {
    id: 108_837_072,
    puid: "wikidata/108837072",
    name: "Nero HFS-CD Compilation",
    extensions: &["nhf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
