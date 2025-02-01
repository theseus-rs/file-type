use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125134441: FileFormat = FileFormat {
    id: 125_134_441,
    puid: "wikidata/125134441",
    name: "YAM Folders",
    extensions: &["folders"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
