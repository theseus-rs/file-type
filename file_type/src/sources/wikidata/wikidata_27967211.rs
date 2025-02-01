use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967211: FileFormat = FileFormat {
    id: 27_967_211,
    puid: "wikidata/27967211",
    name: "Pumatracker module",
    extensions: &["puma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
