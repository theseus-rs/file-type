use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109032204: FileFormat = FileFormat {
    id: 109_032_204,
    puid: "wikidata/109032204",
    name: "Zeiss Vision Image",
    extensions: &["zvi"],
    media_types: &["image/zvi"],
    internal_signatures: &[],
    related_formats: &[],
};
