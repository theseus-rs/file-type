use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440987: FileFormat = FileFormat {
    id: 111_440_987,
    puid: "wikidata/111440987",
    name: "Visual Basic UserDocument",
    extensions: &["dob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
