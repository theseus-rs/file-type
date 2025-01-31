use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111665213: FileFormat = FileFormat {
    id: 111_665_213,
    puid: "wikidata/111665213",
    name: "AbiWord Collaborative File Descriptor",
    extensions: &["abicollab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
