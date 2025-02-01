use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111395876: FileFormat = FileFormat {
    id: 111_395_876,
    puid: "wikidata/111395876",
    name: "Konica format",
    extensions: &["kqp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
