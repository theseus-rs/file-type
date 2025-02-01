use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272274: FileFormat = FileFormat {
    id: 111_272_274,
    puid: "wikidata/111272274",
    name: "Ensoniq KT disk image",
    extensions: &["edk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
