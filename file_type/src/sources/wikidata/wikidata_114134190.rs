use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114134190: FileFormat = FileFormat {
    id: 114_134_190,
    puid: "wikidata/114134190",
    name: "MOPAC dataset format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
