use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009476: FileFormat = FileFormat {
    id: 28_009_476,
    puid: "wikidata/28009476",
    name: "SCF",
    extensions: &["scf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
