use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418430: FileFormat = FileFormat {
    id: 111_418_430,
    puid: "wikidata/111418430",
    name: "Adobe Bridge Collection File",
    extensions: &["collection"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
