use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263214: FileFormat = FileFormat {
    id: 111_263_214,
    puid: "wikidata/111263214",
    name: "3GPP 'AMR interface format 2'",
    extensions: &["cod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
