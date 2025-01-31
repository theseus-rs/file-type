use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009741: FileFormat = FileFormat {
    id: 111_009_741,
    puid: "wikidata/111009741",
    name: "PrintMaster Fax Cover File format",
    extensions: &["fax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
