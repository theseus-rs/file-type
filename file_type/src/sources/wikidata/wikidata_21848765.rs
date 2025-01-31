use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21848765: FileFormat = FileFormat {
    id: 21_848_765,
    puid: "wikidata/21848765",
    name: "BioSemi Data Format",
    extensions: &["bdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
