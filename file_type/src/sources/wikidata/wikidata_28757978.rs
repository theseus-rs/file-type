use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757978: FileFormat = FileFormat {
    id: 28_757_978,
    puid: "wikidata/28757978",
    name: "Precompiled Windows Setup Information File",
    extensions: &["pnf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
