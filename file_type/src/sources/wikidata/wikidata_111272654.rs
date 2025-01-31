use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272654: FileFormat = FileFormat {
    id: 111_272_654,
    puid: "wikidata/111272654",
    name: "ESPS audio file",
    extensions: &["esps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
