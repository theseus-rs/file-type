use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823203: FileFormat = FileFormat {
    id: 27_823_203,
    puid: "wikidata/27823203",
    name: "Synalysis grammar file",
    extensions: &["grammar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
