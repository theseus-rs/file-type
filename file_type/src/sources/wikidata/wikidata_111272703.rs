use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272703: FileFormat = FileFormat {
    id: 111_272_703,
    puid: "wikidata/111272703",
    name: "Floating Point raw 32-bit IEEE data",
    extensions: &["f32"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
