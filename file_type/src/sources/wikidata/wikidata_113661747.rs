use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113661747: FileFormat = FileFormat {
    id: 113_661_747,
    puid: "wikidata/113661747",
    name: "SciFax file",
    extensions: &["sci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
