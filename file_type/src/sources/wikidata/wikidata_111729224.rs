use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111729224: FileFormat = FileFormat {
    id: 111_729_224,
    puid: "wikidata/111729224",
    name: "Document Manager file",
    extensions: &["ddm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
