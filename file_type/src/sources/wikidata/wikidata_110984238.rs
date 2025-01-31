use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110984238: FileFormat = FileFormat {
    id: 110_984_238,
    puid: "wikidata/110984238",
    name: "Ulead Image Sequence",
    extensions: &["uis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
