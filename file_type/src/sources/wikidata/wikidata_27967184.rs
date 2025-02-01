use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967184: FileFormat = FileFormat {
    id: 27_967_184,
    puid: "wikidata/27967184",
    name: "FC-M Packer module",
    extensions: &["fcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
