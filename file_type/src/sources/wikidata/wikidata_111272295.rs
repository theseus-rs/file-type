use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272295: FileFormat = FileFormat {
    id: 111_272_295,
    puid: "wikidata/111272295",
    name: "Ensoniq disk image",
    extensions: &["edx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
