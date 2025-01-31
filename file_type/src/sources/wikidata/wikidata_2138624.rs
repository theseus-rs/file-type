use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2138624: FileFormat = FileFormat {
    id: 2_138_624,
    puid: "wikidata/2138624",
    name: "registry file",
    extensions: &["reg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
