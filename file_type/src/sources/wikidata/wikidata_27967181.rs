use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967181: FileFormat = FileFormat {
    id: 27_967_181,
    puid: "wikidata/27967181",
    name: "Farandole Composer pattern",
    extensions: &["fpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
